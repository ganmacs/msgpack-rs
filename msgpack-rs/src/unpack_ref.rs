// mod value;

// use crate::primitive::*;
// use crate::BufferedRead;
// use crate::{code::Code, unpack_error::UnpackError};

// use std::io::{self, ErrorKind};
// use std::string;

// pub use value::*;

// pub fn unpack_str<'a, R>(reader: &mut R) -> Result<&'a str, UnpackError>
// where
//     R: BufferedRead<'a>,
// {
//     let len = unpack_str_header(reader)?;
//     let buf = reader.fill_buf().map_err(|e| UnpackError::InvalidData(e))?;

//     let buf = &buf[..len];
//     reader.consume(len);
//     use std::error::Error;
//     use std::str;

//     match str::from_utf8(buf) {
//         Ok(s) => Ok(s),
//         Err(err) => Err(UnpackError::InvalidData(io::Error::new(
//             ErrorKind::InvalidData,
//             err.description(),
//         ))),
//     }
//     // Ok(buf)
// }

// use std::io::Read;

// pub fn unpack_str_header<R>(reader: &mut R) -> Result<usize, UnpackError>
// where
//     R: Read,
// {
//     match read_code(reader)? {
//         Code::FixStr(v) => Ok(usize::from(v)),
//         Code::Str8 => read_data_u8(reader).map(usize::from),
//         Code::Str16 => read_data_u16(reader).map(usize::from),
//         Code::Str32 => read_data_u32(reader).map(|v| v as usize),
//         v => Err(UnpackError::TypeMismatch(v)),
//     }
// }
