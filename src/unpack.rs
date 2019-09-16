mod primitive;
pub mod ref_value;
mod value;

use crate::BufferedRead;
use crate::{code::Code, error::UnpackError};
use primitive::*;
use std::io::{self, ErrorKind};

pub use primitive::read_code;
pub use value::*;

pub fn unpack_u8<R: io::Read>(reader: &mut R) -> Result<u8, UnpackError> {
    match read_code(reader)? {
        Code::PosInt(v) => Ok(v),
        Code::Uint8 => read_data_u8(reader),
        e => Err(UnpackError::TypeMismatch(e, "u8".to_string())),
    }
}

pub fn unpack_u16<R: io::Read>(reader: &mut R) -> Result<u16, UnpackError> {
    match read_code(reader)? {
        Code::Uint16 => read_data_u16(reader),
        e => Err(UnpackError::TypeMismatch(e, "u16".to_string())),
    }
}

pub fn unpack_u32<R: io::Read>(reader: &mut R) -> Result<u32, UnpackError> {
    match read_code(reader)? {
        Code::Uint32 => read_data_u32(reader),
        e => Err(UnpackError::TypeMismatch(e, "u32".to_string())),
    }
}

pub fn unpack_u64<R: io::Read>(reader: &mut R) -> Result<u64, UnpackError> {
    match read_code(reader)? {
        Code::Uint64 => read_data_u64(reader),
        e => Err(UnpackError::TypeMismatch(e, "u64".to_string())),
    }
}

pub fn unpack_i8<R: io::Read>(reader: &mut R) -> Result<i8, UnpackError> {
    match read_code(reader)? {
        Code::NegInt(v) => Ok(v),
        Code::Int8 => read_data_i8(reader),
        e => Err(UnpackError::TypeMismatch(e, "i8".to_string())),
    }
}

pub fn unpack_i16<R: io::Read>(reader: &mut R) -> Result<i16, UnpackError> {
    match read_code(reader)? {
        Code::Int16 => read_data_i16(reader),
        e => Err(UnpackError::TypeMismatch(e, "i16".to_string())),
    }
}

pub fn unpack_i32<R: io::Read>(reader: &mut R) -> Result<i32, UnpackError> {
    match read_code(reader)? {
        Code::Int32 => read_data_i32(reader),
        e => Err(UnpackError::TypeMismatch(e, "i32".to_string())),
    }
}

pub fn unpack_i64<R: io::Read>(reader: &mut R) -> Result<i64, UnpackError> {
    match read_code(reader)? {
        Code::Int64 => read_data_i64(reader),
        e => Err(UnpackError::TypeMismatch(e, "i64".to_string())),
    }
}

pub fn unpack_f32<R: io::Read>(reader: &mut R) -> Result<f32, UnpackError> {
    match read_code(reader)? {
        Code::Int32 => read_data_f32(reader),
        e => Err(UnpackError::TypeMismatch(e, "f32".to_string())),
    }
}

pub fn unpack_f64<R: io::Read>(reader: &mut R) -> Result<f64, UnpackError> {
    match read_code(reader)? {
        Code::Int64 => read_data_f64(reader),
        e => Err(UnpackError::TypeMismatch(e, "f64".to_string())),
    }
}

pub fn unpack_bool<R: io::Read>(reader: &mut R) -> Result<bool, UnpackError> {
    match read_code(reader)? {
        Code::True => Ok(true),
        Code::False => Ok(false),
        e => Err(UnpackError::TypeMismatch(e, "bool".to_string())),
    }
}

pub fn unpack_nil<T, R: io::Read>(reader: &mut R) -> Result<Option<T>, UnpackError> {
    match read_code(reader)? {
        Code::Nil => Ok(None),
        e => Err(UnpackError::TypeMismatch(e, "nil".to_string())),
    }
}

pub fn unpack_bin_header<R: io::Read>(reader: &mut R) -> Result<usize, UnpackError> {
    match read_code(reader)? {
        Code::Bin8 => read_data_u8(reader).map(usize::from),
        Code::Bin16 => read_data_u16(reader).map(usize::from),
        Code::Bin32 => read_data_u32(reader).map(|v| v as usize),
        e => Err(UnpackError::TypeMismatch(e, "binary header".to_string())),
    }
}

pub fn unpack_bin_ref<'a, R>(reader: &mut R) -> Result<&'a [u8], UnpackError>
where
    R: BufferedRead<'a>,
{
    ref_value::unpack_bin(reader)
}

pub fn unpack_bin<'a, R>(reader: &mut R) -> Result<Vec<u8>, UnpackError>
where
    R: BufferedRead<'a>,
{
    value::unpack_bin(reader)
}

pub fn unpack_str<R: io::Read>(reader: &mut R) -> Result<String, UnpackError> {
    let len = unpack_str_header(reader)?;
    let buf = value::unpack_str_data(reader, len)?;
    match *buf {
        Ok(_) => Ok(buf.into_string().unwrap()),
        Err((_, e)) => Err(UnpackError::InvalidData(io::Error::new(
            ErrorKind::Other,
            e.to_string(),
        ))),
    }
}

pub fn unpack_str_ref<'a, R>(reader: &mut R) -> Result<&'a str, UnpackError>
where
    R: BufferedRead<'a>,
{
    match *ref_value::unpack_str(reader)? {
        Ok(ref s) => Ok(s),
        Err((_, e)) => Err(UnpackError::InvalidData(io::Error::new(
            ErrorKind::Other,
            e.to_string(),
        ))),
    }
}

pub fn unpack_str_header<R: io::Read>(reader: &mut R) -> Result<usize, UnpackError> {
    match read_code(reader)? {
        Code::FixStr(v) => Ok(usize::from(v)),
        Code::Str8 => read_data_u8(reader).map(usize::from),
        Code::Str16 => read_data_u16(reader).map(usize::from),
        Code::Str32 => read_data_u32(reader).map(|v| v as usize),
        e => Err(UnpackError::TypeMismatch(e, "str header".to_string())),
    }
}

pub fn unpack_array_header<R: io::Read>(reader: &mut R) -> Result<usize, UnpackError> {
    match read_code(reader)? {
        Code::FixArray(v) => Ok(usize::from(v)),
        Code::Array16 => read_data_u16(reader).map(usize::from),
        Code::Array32 => read_data_u32(reader).map(|v| v as usize),
        e => Err(UnpackError::TypeMismatch(e, "array header".to_string())),
    }
}

pub fn unpack_map_header<R: io::Read>(reader: &mut R) -> Result<usize, UnpackError> {
    match read_code(reader)? {
        Code::FixMap(v) => Ok(usize::from(v)),
        Code::Map16 => read_data_u16(reader).map(usize::from),
        Code::Map32 => read_data_u32(reader).map(|v| v as usize),
        e => Err(UnpackError::TypeMismatch(e, "map header".to_string())),
    }
}

pub fn unpack_fixext1<R: io::Read>(reader: &mut R) -> Result<(i8, u8), UnpackError> {
    match read_code(reader)? {
        Code::FixExt1 => {
            let ty = read_data_i8(reader)?;
            let data = read_data_u8(reader)?;
            Ok((ty, data))
        }
        e => Err(UnpackError::TypeMismatch(e, "fixext1 header".to_string())),
    }
}

fn read_fixext_data<R: io::Read>(reader: &mut R, buf: &mut [u8]) -> Result<(), UnpackError> {
    read_data(reader, buf)
}

pub fn unpack_fixext2<R: io::Read>(reader: &mut R) -> Result<(i8, [u8; 2]), UnpackError> {
    match read_code(reader)? {
        Code::FixExt2 => {
            let id = read_data_i8(reader)?;
            let mut buf: [u8; 2] = [0; 2];
            read_fixext_data(reader, &mut buf)?;
            Ok((id, buf))
        }
        e => Err(UnpackError::TypeMismatch(e, "fixext2 header".to_string())),
    }
}

pub fn unpack_fixext4<R: io::Read>(reader: &mut R) -> Result<(i8, [u8; 4]), UnpackError> {
    match read_code(reader)? {
        Code::FixExt4 => {
            let id = read_data_i8(reader)?;
            let mut buf: [u8; 4] = [0; 4];
            read_fixext_data(reader, &mut buf)?;
            Ok((id, buf))
        }
        e => Err(UnpackError::TypeMismatch(e, "fixext4 header".to_string())),
    }
}

pub fn unpack_fixext8<R: io::Read>(reader: &mut R) -> Result<(i8, [u8; 8]), UnpackError> {
    match read_code(reader)? {
        Code::FixExt8 => {
            let id = read_data_i8(reader)?;
            let mut buf: [u8; 8] = [0; 8];
            read_fixext_data(reader, &mut buf)?;
            Ok((id, buf))
        }
        e => Err(UnpackError::TypeMismatch(e, "fixext8 header".to_string())),
    }
}

pub fn unpack_fixext16<R: io::Read>(reader: &mut R) -> Result<(i8, [u8; 16]), UnpackError> {
    match read_code(reader)? {
        Code::FixExt16 => {
            let id = read_data_i8(reader)?;
            let mut buf: [u8; 16] = [0; 16];
            read_fixext_data(reader, &mut buf)?;
            Ok((id, buf))
        }
        e => Err(UnpackError::TypeMismatch(e, "fixext16 header".to_string())),
    }
}

pub fn unpack_ext_header<R: io::Read>(reader: &mut R) -> Result<(u32, i8), UnpackError> {
    let len = match read_code(reader)? {
        Code::FixExt1 => 1,
        Code::FixExt2 => 2,
        Code::FixExt4 => 4,
        Code::FixExt8 => 8,
        Code::FixExt16 => 16,
        Code::Ext8 => read_data_u8(reader).map(u32::from)?,
        Code::Ext16 => read_data_u16(reader).map(u32::from)?,
        Code::Ext32 => read_data_u32(reader)?,
        e => return Err(UnpackError::TypeMismatch(e, "ext header".to_string())),
    };
    let ty = read_data_i8(reader)?;
    Ok((len, ty))
}

pub fn unpack_timestamp32<R: io::Read>(reader: &mut R) -> Result<(u32), UnpackError> {
    match read_code(reader)? {
        Code::FixExt4 => {
            let type_tag = read_data_i8(reader)?;
            if type_tag == -1 {
                read_data_u32(reader)
            } else {
                Err(UnpackError::InvalidData(io::Error::new(
                    ErrorKind::Other,
                    format!("Timestamp32 expects -1 but {:?}", type_tag),
                )))
            }
        }
        e => Err(UnpackError::TypeMismatch(
            e,
            "timestamp32 header".to_string(),
        )),
    }
}

const TIMESTAMP64_SEC_MASK: u64 = (1 << 35) - 1;
const TIMESTAMP64_NSEC_MASK: u32 = (1 << 31) - 1;

pub fn unpack_timestamp64<R: io::Read>(reader: &mut R) -> Result<(u64, u32), UnpackError> {
    match read_code(reader)? {
        Code::FixExt8 => {
            let type_tag = read_data_i8(reader)?;
            if type_tag == -1 {
                let v = read_data_u64(reader)?;
                let sec = v | TIMESTAMP64_SEC_MASK;
                let nsec = (v >> 34) as u32 | TIMESTAMP64_NSEC_MASK;
                Ok((sec, nsec))
            } else {
                Err(UnpackError::InvalidData(io::Error::new(
                    ErrorKind::Other,
                    format!("Timestamp64 expects -1 but {:?}", type_tag),
                )))
            }
        }
        v => Err(UnpackError::TypeMismatch(
            v,
            "timestamp64 header".to_string(),
        )),
    }
}

pub fn unpack_timestamp96<R: io::Read>(reader: &mut R) -> Result<(i64, u32), UnpackError> {
    match read_code(reader)? {
        Code::Ext8 => {
            let type_tag = read_data_i8(reader)?;
            let size = read_data_u8(reader)?;
            if type_tag == -1 && size == 12 {
                let nsec = read_data_u32(reader)?;
                let sec = read_data_i64(reader)?;
                Ok((sec, nsec))
            } else {
                Err(UnpackError::InvalidData(io::Error::new(
                    ErrorKind::Other,
                    format!("Timestamp96 expects -1 but {:?}", type_tag),
                )))
            }
        }
        v => Err(UnpackError::TypeMismatch(
            v,
            "timestamp96 header".to_string(),
        )),
    }
}
