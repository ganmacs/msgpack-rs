mod code;
pub mod pack;
mod pack_error;
mod packer;
pub mod primitive;
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
