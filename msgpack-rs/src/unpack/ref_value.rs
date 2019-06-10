use crate::primitive::*;
use crate::{code, unpack, value::RefValue, BufferedRead, UnpackError};

use std::error::Error;
use std::io::{self, ErrorKind, Read};
use std::str;

pub fn unpack_ary_data<'a, R>(reader: &mut R, len: usize) -> Result<Vec<RefValue<'a>>, UnpackError>
where
    R: BufferedRead<'a>,
{
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(unpack_value_ref(reader)?);
    }

    Ok(vec)
}

pub fn unpack_ary<'a, R>(reader: &mut R) -> Result<Vec<RefValue<'a>>, UnpackError>
where
    R: BufferedRead<'a>,
{
    let len = unpack::unpack_ary_header(reader)?;
    unpack_ary_data(reader, len)
}

pub fn unpack_map_data<'a, R>(
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

pub fn unpack_map<'a, R>(reader: &mut R) -> Result<Vec<(RefValue<'a>, RefValue<'a>)>, UnpackError>
where
    R: BufferedRead<'a>,
{
    let len = unpack::unpack_map_header(reader)?;
    unpack_map_data(reader, len)
}

fn unpack_str_data<'a, R>(reader: &mut R, len: usize) -> Result<&'a str, UnpackError>
where
    R: BufferedRead<'a>,
{
    let buf = unpack_bin_data(reader, len)?;
    match str::from_utf8(buf) {
        Ok(s) => Ok(s),
        Err(err) => Err(UnpackError::InvalidData(io::Error::new(
            ErrorKind::InvalidData,
            err.description(),
        ))),
    }
}

pub fn unpack_str<'a, R>(reader: &mut R) -> Result<&'a str, UnpackError>
where
    R: BufferedRead<'a>,
{
    let len = unpack::unpack_str_header(reader)?;
    unpack_str_data(reader, len)
}

#[test]
fn test_unpack_str() {
    let v = vec![0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let mut cur = io::Cursor::new(v.as_ref());
    let ret = unpack_str(&mut cur).unwrap();
    assert_eq!(ret, "hello");
}

fn unpack_bin_data<'a, R>(reader: &mut R, len: usize) -> Result<&'a [u8], UnpackError>
where
    R: BufferedRead<'a>,
{
    let buf = reader.fill_buf().map_err(|e| UnpackError::InvalidData(e))?;
    if len > buf.len() {
        return Err(UnpackError::InvalidData(io::Error::new(
            ErrorKind::UnexpectedEof,
            "Unexpected EOF",
        )));
    }

    let buf = &buf[..len];
    reader.consume(len);
    Ok(buf)
}

pub fn unpack_bin<'a, R>(reader: &mut R) -> Result<&'a [u8], UnpackError>
where
    R: BufferedRead<'a>,
{
    let len = unpack::unpack_bin_header(reader)?;
    unpack_bin_data(reader, len)
}

#[test]
fn test_unpack_bin() {
    let v = vec![0xc4, 0x03, 0x61, 0x61, 0x61];
    let mut cur = io::Cursor::new(v.as_ref());
    let ret = unpack_bin(&mut cur).unwrap();
    assert_eq!(ret, &[0x61, 0x61, 0x61]);
}

fn unpack_ext_type_data<'a, R>(reader: &mut R, len: usize) -> Result<(i8, &'a [u8]), UnpackError>
where
    R: BufferedRead<'a>,
{
    let ty = read_data_i8(reader)?;
    let bin = unpack_bin_data(reader, len)?;
    Ok((ty, bin))
}

pub fn unpack_value_ref<'a, R>(reader: &mut R) -> Result<RefValue<'a>, UnpackError>
where
    R: BufferedRead<'a>,
{
    use code::Code;

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
        Code::FixStr(len) => RefValue::String(unpack_str_data(reader, usize::from(len))?),
        Code::Str8 | Code::Str16 | Code::Str32 => RefValue::String(unpack_str(reader)?),
        Code::Bin8 | Code::Bin16 | Code::Bin32 => RefValue::Binary(unpack_bin(reader)?),
        Code::FixArray(len) => RefValue::Array(unpack_ary_data(reader, len as usize)?),
        Code::Array16 | Code::Array32 => RefValue::Array(unpack_ary(reader)?),
        Code::FixMap(len) => RefValue::Map(unpack_map_data(reader, len as usize)?),
        Code::Map16 | Code::Map32 => RefValue::Map(unpack_map(reader)?),
        Code::FixExt1 => {
            let (ty, vec) = unpack_ext_type_data(reader, 1)?;
            RefValue::Extension(ty, vec)
        }
        Code::FixExt2 => {
            let (ty, vec) = unpack_ext_type_data(reader, 2)?;
            RefValue::Extension(ty, vec)
        }
        Code::FixExt4 => {
            let (ty, vec) = unpack_ext_type_data(reader, 4)?;
            RefValue::Extension(ty, vec)
        }
        Code::FixExt8 => {
            let (ty, vec) = unpack_ext_type_data(reader, 8)?;
            RefValue::Extension(ty, vec)
        }
        Code::FixExt16 => {
            let (ty, vec) = unpack_ext_type_data(reader, 16)?;
            RefValue::Extension(ty, vec)
        }
        Code::Ext8 => {
            let len = usize::from(read_data_u8(reader)?);
            let (ty, vec) = unpack_ext_type_data(reader, len)?;
            RefValue::Extension(ty, vec)
        }
        Code::Ext16 => {
            let len = usize::from(read_data_u16(reader)?);
            let (ty, vec) = unpack_ext_type_data(reader, len)?;
            RefValue::Extension(ty, vec)
        }
        Code::Ext32 => {
            let len = read_data_u32(reader)? as usize;
            let (ty, vec) = unpack_ext_type_data(reader, len)?;
            RefValue::Extension(ty, vec)
        }
        // Code::Reserved => unreachable!(),
        _ => unreachable!(),
    };

    Ok(val)
}
