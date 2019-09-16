mod code;
pub mod de;
pub mod pack;
mod pack_error;
mod packer;
mod ser;
pub mod unpack;
mod unpack_error;
mod unpacker;
mod value;

pub use byteorder::ReadBytesExt;
pub use pack::*;
pub use pack_error::PackError;
pub use packer::Packer;
pub use unpack::*;
pub use unpack_error::UnpackError;
pub use unpacker::{InnerBuf, RefUnpacker, Unpacker};
pub use value::{RefValue, Value};

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
