use crate::{code, value::ValueRef, UnpackError};

use std::error::Error;
use std::io::Cursor;
use std::io::{self, BufRead, ErrorKind, Read};
use std::str;

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

fn unpack_str_data<'a, R>(rd: &mut R, len: usize) -> Result<&'a str, UnpackError>
where
    R: BufferedRead<'a>,
{
    let buf = read_bin_data(rd, len)?;
    match str::from_utf8(buf) {
        Ok(s) => Ok(s),
        Err(err) => Err(UnpackError::InvalidData(io::Error::new(
            ErrorKind::InvalidData,
            err.description(),
        ))),
    }
}

#[test]
fn test_unpack_str_data() {
    let v = vec![0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let mut cur = io::Cursor::new(v.as_ref());
    let ret = unpack_str_data(&mut cur, 5).unwrap();
    assert_eq!(ret, "hello");
}

fn unpack_bin_data<'a, R>(rd: &mut R, len: usize) -> Result<&'a [u8], UnpackError>
where
    R: BufferedRead<'a>,
{
    let buf = rd.fill_buf().map_err(|e| UnpackError::InvalidData(e))?;
    if len > buf.len() {
        return Err(UnpackError::InvalidData(io::Error::new(
            ErrorKind::UnexpectedEof,
            "Unexpected EOF",
        )));
    }

    let buf = &buf[..len];
    rd.consume(len);
    Ok(buf)
}
