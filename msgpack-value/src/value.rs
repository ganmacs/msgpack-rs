pub mod float;
pub mod from;
pub mod integer;
pub mod utf8_string;

pub use float::Float;
pub use integer::Integer;
pub use utf8_string::{Utf8String, Utf8StringRef};

// use crate::unpack::primitive::*;
// use crate::{code, unpack, value, value::Value, UnpackError};
// use std::io;

// pub fn unpack_bin_data<R: io::Read>(reader: &mut R, len: usize) -> Result<Vec<u8>, UnpackError> {
//     let mut buf = Vec::with_capacity(len);
//     buf.resize(len as usize, 0u8);
//     read_data(reader, &mut buf[..])?;
//     Ok(buf)
// }

// pub fn unpack_str_data<R: io::Read>(
//     reader: &mut R,
//     len: usize,
// ) -> Result<value::Utf8String, UnpackError> {
//     let buf = unpack_bin_data(reader, len)?;
//     Ok(value::Utf8String::from(buf))
// }

// fn unpack_ext_type_data<R: io::Read>(
//     reader: &mut R,
//     len: usize,
// ) -> Result<(i8, Vec<u8>), UnpackError> {
//     let ty = read_data_i8(reader)?;
//     let vec = unpack_bin_data(reader, len)?;
//     Ok((ty, vec))
// }

// pub fn unpack_bin<R>(reader: &mut R) -> Result<Vec<u8>, UnpackError>
// where
//     R: io::Read,
// {
//     let len = unpack::unpack_bin_header(reader)?;
//     unpack_bin_data(reader, len)
// }
