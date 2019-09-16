use crate::unpack::primitive::*;
use crate::{code, unpack, value, value::Value, UnpackError};
use std::io;

pub fn unpack_bin_data<R: io::Read>(reader: &mut R, len: usize) -> Result<Vec<u8>, UnpackError> {
    let mut buf = Vec::with_capacity(len);
    buf.resize(len as usize, 0u8);
    read_data(reader, &mut buf[..])?;
    Ok(buf)
}

pub fn unpack_array_data<R: io::Read>(
    reader: &mut R,
    len: usize,
) -> Result<Vec<Value>, UnpackError> {
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(unpack_value(reader)?);
    }

    Ok(vec)
}

pub fn unpack_str_data<R: io::Read>(
    reader: &mut R,
    len: usize,
) -> Result<value::Utf8String, UnpackError> {
    let buf = unpack_bin_data(reader, len)?;
    Ok(value::Utf8String::from(buf))
}

pub fn unpack_map_data<R: io::Read>(
    reader: &mut R,
    len: usize,
) -> Result<Vec<(Value, Value)>, UnpackError> {
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push((unpack_value(reader)?, unpack_value(reader)?));
    }
    Ok(vec)
}

fn unpack_ext_type_data<R: io::Read>(
    reader: &mut R,
    len: usize,
) -> Result<(i8, Vec<u8>), UnpackError> {
    let ty = read_data_i8(reader)?;
    let vec = unpack_bin_data(reader, len)?;
    Ok((ty, vec))
}

pub fn unpack_bin<R>(reader: &mut R) -> Result<Vec<u8>, UnpackError>
where
    R: io::Read,
{
    let len = unpack::unpack_bin_header(reader)?;
    unpack_bin_data(reader, len)
}

pub fn unpack_value<R>(reader: &mut R) -> Result<Value, UnpackError>
where
    R: io::Read,
{
    use code::Code;
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
            Value::Binary(unpack_bin_data(reader, len)?)
        }
        Code::Bin16 => {
            let len = usize::from(read_data_u16(reader)?);
            Value::Binary(unpack_bin_data(reader, len)?)
        }
        Code::Bin32 => {
            let len = read_data_u32(reader)? as usize;
            Value::Binary(unpack_bin_data(reader, len)?)
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
