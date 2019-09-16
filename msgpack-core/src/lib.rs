pub mod code;
mod error;
pub mod pack;
mod packer;
pub mod unpack;
mod unpacker;

pub use byteorder::ReadBytesExt;
pub use error::{PackError, UnpackError};
pub use pack::*;
pub use packer::Packer;
pub use unpack::*;
pub use unpacker::{InnerBuf, Unpacker};

use std::io::{self, Cursor, Read};

pub trait BufferedRead<'a>: Read {
    // having same lifetime as BorrowRead
    fn fill_buf(&self) -> io::Result<&'a [u8]>;

    fn consume(&mut self, len: usize);
}

impl<'a> BufferedRead<'a> for &'a [u8] {
    fn fill_buf(&self) -> io::Result<&'a [u8]> {
        Ok(self)
    }

    fn consume(&mut self, len: usize) {
        *self = &(*self)[len..];
    }
}

impl<'a> BufferedRead<'a> for Cursor<&'a [u8]> {
    fn fill_buf(&self) -> io::Result<&'a [u8]> {
        let len = std::cmp::min(self.position(), self.get_ref().len() as u64);
        Ok(&self.get_ref()[len as usize..])
    }

    fn consume(&mut self, len: usize) {
        let pos = self.position();
        self.set_position(pos + len as u64);
    }
}

pub trait MessagePacker {
    fn pack_nil(&mut self) -> Result<(), PackError>;
    fn pack_uint(&mut self, v: u64) -> Result<(), PackError>;
    fn pack_u8(&mut self, v: u8) -> Result<(), PackError>;
    fn pack_from_u8(&mut self, v: u8) -> Result<(), PackError>;
    fn pack_from_u16(&mut self, v: u16) -> Result<(), PackError>;
    fn pack_u16(&mut self, v: u16) -> Result<(), PackError>;
    fn pack_from_u32(&mut self, v: u32) -> Result<(), PackError>;
    fn pack_u32(&mut self, v: u32) -> Result<(), PackError>;
    fn pack_from_u64(&mut self, v: u64) -> Result<(), PackError>;
    fn pack_u64(&mut self, v: u64) -> Result<(), PackError>;
    fn pack_int(&mut self, v: i64) -> Result<(), PackError>;
    fn pack_from_i8(&mut self, v: i8) -> Result<(), PackError>;
    fn pack_i8(&mut self, v: i8) -> Result<(), PackError>;
    fn pack_from_i16(&mut self, v: i16) -> Result<(), PackError>;
    fn pack_i16(&mut self, v: i16) -> Result<(), PackError>;
    fn pack_from_i32(&mut self, v: i32) -> Result<(), PackError>;
    fn pack_i32(&mut self, v: i32) -> Result<(), PackError>;
    fn pack_from_i64(&mut self, v: i64) -> Result<(), PackError>;
    fn pack_i64(&mut self, v: i64) -> Result<(), PackError>;
    fn pack_f32(&mut self, v: f32) -> Result<(), PackError>;
    fn pack_f64(&mut self, v: f64) -> Result<(), PackError>;
    fn pack_bool(&mut self, v: bool) -> Result<(), PackError>;
    fn pack_str(&mut self, v: &str) -> Result<(), PackError>;
    fn pack_bin(&mut self, v: &[u8]) -> Result<(), PackError>;
    fn pack_array_header(&mut self, len: usize) -> Result<(), PackError>;
    fn pack_map_header(&mut self, len: usize) -> Result<(), PackError>;
    fn pack_ext_header(&mut self, ext_type: i8, len: usize) -> Result<(), PackError>;
    // this method is used after pack_ext_header
    fn write_payload(&mut self, v: &[u8]) -> Result<(), PackError>;
    fn flush(&mut self) -> Result<(), PackError>;
}

pub trait MessageUnpacker {
    fn unpack_u8(&mut self) -> Result<u8, UnpackError>;
    fn unpack_u16(&mut self) -> Result<u16, UnpackError>;
    fn unpack_u32(&mut self) -> Result<u32, UnpackError>;
    fn unpack_u64(&mut self) -> Result<u64, UnpackError>;
    fn unpack_i8(&mut self) -> Result<i8, UnpackError>;
    fn unpack_i16(&mut self) -> Result<i16, UnpackError>;
    fn unpack_i32(&mut self) -> Result<i32, UnpackError>;
    fn unpack_i64(&mut self) -> Result<i64, UnpackError>;
    fn unpack_nil<T>(&mut self) -> Result<Option<T>, UnpackError>;
    fn unpack_bool(&mut self) -> Result<bool, UnpackError>;
    fn unpack_string(&mut self) -> Result<String, UnpackError>;
    fn unpack_str_header(&mut self) -> Result<usize, UnpackError>;
    fn unpack_array_header(&mut self) -> Result<usize, UnpackError>;
    fn unpack_map_header(&mut self) -> Result<usize, UnpackError>;
    fn unpack_bin_header(&mut self) -> Result<usize, UnpackError>;
    fn unpack_fixext1(&mut self) -> Result<(i8, u8), UnpackError>;
    fn unpack_fixext2(&mut self) -> Result<(i8, [u8; 2]), UnpackError>;
    fn unpack_fixext4(&mut self) -> Result<(i8, [u8; 4]), UnpackError>;
    fn unpack_fixext8(&mut self) -> Result<(i8, [u8; 8]), UnpackError>;
    fn unpack_fixext16(&mut self) -> Result<(i8, [u8; 16]), UnpackError>;
}
