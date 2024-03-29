use msgpack::unpack::primitive::*;
use msgpack::unpack::*;
use msgpack::{code::Code, BufferedRead, UnpackError};
use std::io;

use crate::{value, RefValue, Value};

fn unpack_str_data<R: io::Read>(
    reader: &mut R,
    len: usize,
) -> Result<value::Utf8String, UnpackError> {
    let buf = unpack_data(reader, len)?;
    Ok(value::Utf8String::from(buf))
}

fn unpack_map_data<R: io::Read>(
    reader: &mut R,
    len: usize,
) -> Result<Vec<(Value, Value)>, UnpackError> {
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push((unpack_value(reader)?, unpack_value(reader)?));
    }
    Ok(vec)
}

fn unpack_array_data<R: io::Read>(reader: &mut R, len: usize) -> Result<Vec<Value>, UnpackError> {
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(unpack_value(reader)?);
    }

    Ok(vec)
}

pub fn unpack_value<R>(reader: &mut R) -> Result<Value, UnpackError>
where
    R: io::Read,
{
    let val = match read_code(reader)? {
        Code::Nil => Value::Nil,
        Code::True => Value::from(true),
        Code::False => Value::from(false),
        Code::PosInt(v) => Value::from(v),
        Code::Uint8 => Value::from(read_data_u8(reader)?),
        Code::Uint16 => Value::from(read_data_u16(reader)?),
        Code::Uint32 => Value::from(read_data_u32(reader)?),
        Code::Uint64 => Value::from(read_data_u64(reader)?),
        Code::NegInt(v) => Value::from(v),
        Code::Int8 => Value::from(read_data_i8(reader)?),
        Code::Int16 => Value::from(read_data_i16(reader)?),
        Code::Int32 => Value::from(read_data_i32(reader)?),
        Code::Int64 => Value::from(read_data_i64(reader)?),
        Code::Float32 => Value::from(read_data_f32(reader)?),
        Code::Float64 => Value::from(read_data_f64(reader)?),
        Code::Bin8 => {
            let len = usize::from(read_data_u8(reader)?);
            Value::Binary(unpack_data(reader, len)?)
        }
        Code::Bin16 => {
            let len = usize::from(read_data_u16(reader)?);
            Value::Binary(unpack_data(reader, len)?)
        }
        Code::Bin32 => {
            let len = read_data_u32(reader)? as usize;
            Value::Binary(unpack_data(reader, len)?)
        }
        Code::FixStr(len) => Value::String(unpack_str_data(reader, usize::from(len))?),
        Code::Str8 => {
            let len = usize::from(read_data_u8(reader)?);
            Value::String(unpack_str_data(reader, len)?)
        }
        Code::Str16 => {
            let len = usize::from(read_data_u16(reader)?);
            Value::String(unpack_str_data(reader, len)?)
        }
        Code::Str32 => {
            let len = read_data_u32(reader)?;
            Value::String(unpack_str_data(reader, len as usize)?)
        }
        Code::FixArray(len) => Value::Array(unpack_array_data(reader, len as usize)?),
        Code::Array16 => {
            let len = usize::from(read_data_u16(reader)?);
            Value::Array(unpack_array_data(reader, len)?)
        }
        Code::Array32 => {
            let len = read_data_u32(reader)? as usize;
            Value::Array(unpack_array_data(reader, len)?)
        }
        Code::FixMap(len) => Value::Map(unpack_map_data(reader, len as usize)?),
        Code::Map16 => {
            let len = usize::from(read_data_u16(reader)?);
            Value::Map(unpack_map_data(reader, len)?)
        }
        Code::Map32 => {
            let len = read_data_u16(reader)? as usize;
            Value::Map(unpack_map_data(reader, len)?)
        }
        Code::FixExt1 => {
            let (ty, vec) = unpack_ext_type_data(reader, 1)?;
            Value::Extension(ty, vec)
        }
        Code::FixExt2 => {
            let (ty, vec) = unpack_ext_type_data(reader, 2)?;
            Value::Extension(ty, vec)
        }
        Code::FixExt4 => {
            let (ty, vec) = unpack_ext_type_data(reader, 4)?;
            Value::Extension(ty, vec)
        }
        Code::FixExt8 => {
            let (ty, vec) = unpack_ext_type_data(reader, 8)?;
            Value::Extension(ty, vec)
        }
        Code::FixExt16 => {
            let (ty, vec) = unpack_ext_type_data(reader, 16)?;
            Value::Extension(ty, vec)
        }
        Code::Ext8 => {
            let len = usize::from(read_data_u8(reader)?);
            let (ty, vec) = unpack_ext_type_data(reader, len)?;
            Value::Extension(ty, vec)
        }
        Code::Ext16 => {
            let len = usize::from(read_data_u16(reader)?);
            let (ty, vec) = unpack_ext_type_data(reader, len)?;
            Value::Extension(ty, vec)
        }
        Code::Ext32 => {
            let len = read_data_u32(reader)? as usize;
            let (ty, vec) = unpack_ext_type_data(reader, len)?;
            Value::Extension(ty, vec)
        }
        Code::Reserved => unreachable!(),
    };

    Ok(val)
}

// use std::io::{self, ErrorKind};

fn unpack_array_data_ref<'a, R>(
    reader: &mut R,
    len: usize,
) -> Result<Vec<RefValue<'a>>, UnpackError>
where
    R: BufferedRead<'a>,
{
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(unpack_value_ref(reader)?);
    }

    Ok(vec)
}

fn unpack_array_ref<'a, R>(reader: &mut R) -> Result<Vec<RefValue<'a>>, UnpackError>
where
    R: BufferedRead<'a>,
{
    let len = unpack_array_header(reader)?;
    unpack_array_data_ref(reader, len)
}

fn unpack_map_data_ref<'a, R>(
    reader: &mut R,
    len: usize,
) -> Result<Vec<(RefValue<'a>, RefValue<'a>)>, UnpackError>
where
    R: BufferedRead<'a>,
{
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push((unpack_value_ref(reader)?, unpack_value_ref(reader)?));
    }

    Ok(vec)
}

fn unpack_map_ref<'a, R>(reader: &mut R) -> Result<Vec<(RefValue<'a>, RefValue<'a>)>, UnpackError>
where
    R: BufferedRead<'a>,
{
    let len = unpack_map_header(reader)?;
    unpack_map_data_ref(reader, len)
}

fn unpack_str_data_ref<'a, R>(
    reader: &mut R,
    len: usize,
) -> Result<value::Utf8StringRef<'a>, UnpackError>
where
    R: BufferedRead<'a>,
{
    let buf = unpack_data_ref(reader, len)?;
    Ok(value::Utf8StringRef::from(buf))
}

// pub fn unpack_str<'a, R>(reader: &mut R) -> Result<value::Utf8StringRef<'a>, UnpackError>
// where
//     R: BufferedRead<'a>,
// {
//     let len = unpack::unpack_str_header(reader)?;
//     unpack_str_data(reader, len)
// }

// #[test]
// fn test_unpack_str() {
//     let v = vec![0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
//     let mut cur = io::Cursor::new(v.as_ref());
//     let ret = unpack_str(&mut cur).unwrap();
//     assert_eq!(*ret, Ok("hello"));
// }

// fn unpack_bin_data<'a, R>(reader: &mut R, len: usize) -> Result<&'a [u8], UnpackError>
// where
//     R: BufferedRead<'a>,
// {
//     let buf = reader.fill_buf().map_err(|e| UnpackError::InvalidData(e))?;
//     if len > buf.len() {
//         return Err(UnpackError::InvalidData(io::Error::new(
//             ErrorKind::UnexpectedEof,
//             "Unexpected EOF",
//         )));
//     }

//     let buf = &buf[..len];
//     reader.consume(len);
//     Ok(buf)
// }

// pub fn unpack_bin<'a, R>(reader: &mut R) -> Result<&'a [u8], UnpackError>
// where
//     R: BufferedRead<'a>,
// {
//     let len = unpack::unpack_bin_header(reader)?;
//     unpack_bin_data(reader, len)
// }

// #[test]
// fn test_unpack_bin() {
//     let v = vec![0xc4, 0x03, 0x61, 0x61, 0x61];
//     let mut cur = io::Cursor::new(v.as_ref());
//     let ret = unpack_bin(&mut cur).unwrap();
//     assert_eq!(ret, &[0x61, 0x61, 0x61]);
// }

const TIMESTAMP64_SEC_MASK: u64 = (1 << 35) - 1;
const TIMESTAMP64_NSEC_MASK: u32 = (1 << 31) - 1;

pub fn unpack_value_ref<'a, R>(reader: &mut R) -> Result<RefValue<'a>, UnpackError>
where
    R: BufferedRead<'a>,
{
    let val = match read_code(reader)? {
        Code::Nil => RefValue::Nil,
        Code::True => RefValue::from(true),
        Code::False => RefValue::from(false),
        Code::PosInt(v) => RefValue::from(v),
        Code::Uint8 => RefValue::from(read_data_u8(reader)?),
        Code::Uint16 => RefValue::from(read_data_u16(reader)?),
        Code::Uint32 => RefValue::from(read_data_u32(reader)?),
        Code::Uint64 => RefValue::from(read_data_u64(reader)?),
        Code::NegInt(v) => RefValue::from(v),
        Code::Int8 => RefValue::from(read_data_i8(reader)?),
        Code::Int16 => RefValue::from(read_data_i16(reader)?),
        Code::Int32 => RefValue::from(read_data_i32(reader)?),
        Code::Int64 => RefValue::from(read_data_i64(reader)?),
        Code::Float32 => RefValue::from(read_data_f32(reader)?),
        Code::Float64 => RefValue::from(read_data_f64(reader)?),
        Code::FixStr(len) => RefValue::String(unpack_str_data_ref(reader, usize::from(len))?),
        Code::Str8 => {
            let len = usize::from(read_data_u8(reader)?);
            RefValue::String(unpack_str_data_ref(reader, len)?)
        }
        Code::Str16 => {
            let len = usize::from(read_data_u16(reader)?);
            RefValue::String(unpack_str_data_ref(reader, len)?)
        }
        Code::Str32 => {
            let len = read_data_u32(reader)? as usize;
            RefValue::String(unpack_str_data_ref(reader, len)?)
        }
        Code::Bin8 => {
            let len = usize::from(read_data_u8(reader)?);
            RefValue::Binary(unpack_data_ref(reader, len)?)
        }
        Code::Bin16 => {
            let len = usize::from(read_data_u16(reader)?);
            RefValue::Binary(unpack_data_ref(reader, len)?)
        }
        Code::Bin32 => {
            let len = read_data_u32(reader)? as usize;
            RefValue::Binary(unpack_data_ref(reader, len)?)
        }
        Code::FixArray(len) => RefValue::Array(unpack_array_data_ref(reader, len as usize)?),
        Code::Array16 => {
            let len = usize::from(read_data_u16(reader)?);
            RefValue::Array(unpack_array_data_ref(reader, len)?)
        }
        Code::Array32 => {
            let len = read_data_u32(reader)? as usize;
            RefValue::Array(unpack_array_data_ref(reader, len)?)
        }
        Code::FixMap(len) => RefValue::Map(unpack_map_data_ref(reader, len as usize)?),
        Code::Map16 => {
            let len = read_data_u16(reader)? as usize;
            RefValue::Map(unpack_map_data_ref(reader, len)?)
        }
        Code::Map32 => {
            let len = read_data_u16(reader)? as usize;
            RefValue::Map(unpack_map_data_ref(reader, len)?)
        }
        Code::FixExt1 => {
            let (ty, vec) = unpack_ext_type_data_ref(reader, 1)?;
            RefValue::Extension(ty, vec)
        }
        Code::FixExt2 => {
            let (ty, vec) = unpack_ext_type_data_ref(reader, 2)?;
            RefValue::Extension(ty, vec)
        }
        Code::FixExt4 => {
            let ty = read_data_i8(reader)?;

            // TODO: refactor
            if ty == -1 {
                let v: u32 = read_data_u32(reader)?;
                RefValue::Timestamp(v as i64, 0)
            } else {
                let buf = unpack_data_ref(reader, 4)?;
                RefValue::Extension(ty, buf)
            }
        }
        Code::FixExt8 => {
            let ty = read_data_i8(reader)?;

            // TODO: refactor
            if ty == -1 {
                let v = read_data_u64(reader)?;
                let sec = v | TIMESTAMP64_SEC_MASK;
                let nsec = (v >> 34) as u32 | TIMESTAMP64_NSEC_MASK;
                RefValue::Timestamp(sec as i64, nsec)
            } else {
                let buf = unpack_data_ref(reader, 8)?;
                RefValue::Extension(ty, buf)
            }
        }
        Code::FixExt16 => {
            let (ty, vec) = unpack_ext_type_data_ref(reader, 16)?;
            RefValue::Extension(ty, vec)
        }
        Code::Ext8 => {
            // TODO: refactor
            let len = usize::from(read_data_u8(reader)?);
            let ty = read_data_i8(reader)?;
            if len == 12 && ty == -1 {
                let nsec = read_data_u32(reader)?;
                let sec = read_data_i64(reader)?;
                RefValue::Timestamp(sec as i64, nsec)
            } else {
                let buf = unpack_data_ref(reader, 4)?;
                RefValue::Extension(ty, buf)
            }
        }
        Code::Ext16 => {
            let len = usize::from(read_data_u16(reader)?);
            let (ty, vec) = unpack_ext_type_data_ref(reader, len)?;
            RefValue::Extension(ty, vec)
        }
        Code::Ext32 => {
            let len = read_data_u32(reader)? as usize;
            let (ty, vec) = unpack_ext_type_data_ref(reader, len)?;
            RefValue::Extension(ty, vec)
        }
        // Code::Reserved => unreachable!(),
        _ => unreachable!(),
    };

    Ok(val)
}
