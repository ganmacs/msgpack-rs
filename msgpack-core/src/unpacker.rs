use crate::{unpack, MessageUnpacker, UnpackError};

use bytes::{Buf, BytesMut};
use std::{cmp, io};

#[derive(Debug)]
pub struct Unpacker<R> {
    inner: R,
}

impl<T> Unpacker<T>
where
    T: io::Read,
{
    pub fn from_reader(inner: T) -> Self {
        Unpacker { inner }
    }
}

#[derive(Debug)]
pub struct InnerBuf(io::Cursor<bytes::BytesMut>);

impl io::Write for InnerBuf {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.get_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Read for InnerBuf {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.0.get_ref().len() == 0 {
            use std::io::ErrorKind;
            return Err(io::Error::new(
                ErrorKind::UnexpectedEof,
                "There is no more read data",
            ));
        }

        let len = cmp::min(self.0.get_ref().len(), buf.len());
        Buf::copy_to_slice(&mut self.0, &mut buf[0..len]);
        Ok(len)
    }
}

impl InnerBuf {
    pub fn new() -> Self {
        Self(io::Cursor::new(BytesMut::new()))
    }

    pub fn as_ref(&self) -> &io::Cursor<bytes::BytesMut> {
        &self.0
    }
}

impl Unpacker<InnerBuf> {
    pub fn new() -> Self {
        Unpacker {
            inner: InnerBuf::new(),
        }
    }
}

impl<T> io::Write for Unpacker<T>
where
    T: io::Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<T> io::Read for Unpacker<T>
where
    T: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

// impl<'a, R> Unpacker<R>
// where
//     R: io::Write,
// {
//     pub fn feed_slice(&mut self, slice: &[u8]) -> io::Result<usize> {
//         self.inner.write(slice)
//     }
// }

// impl<'a, R> Unpacker<R>
// where
//     R: io::Write + io::Read,
// {
//     pub fn feed_each<F>(&mut self, slice: &[u8], f: F) -> io::Result<usize>
//     where
//         F: Fn(Value) -> (),
//     {
//         let r = self.feed_slice(slice)?;
//         for val in self.iter() {
//             f(val)
//         }

//         Ok(r)
//     }
// }

impl<R: io::Read> MessageUnpacker for Unpacker<R> {
    fn unpack_u8(&mut self) -> Result<u8, UnpackError> {
        unpack::unpack_u8(&mut self.inner)
    }

    fn unpack_u16(&mut self) -> Result<u16, UnpackError> {
        unpack::unpack_u16(&mut self.inner)
    }

    fn unpack_u32(&mut self) -> Result<u32, UnpackError> {
        unpack::unpack_u32(&mut self.inner)
    }

    fn unpack_u64(&mut self) -> Result<u64, UnpackError> {
        unpack::unpack_u64(&mut self.inner)
    }

    fn unpack_i8(&mut self) -> Result<i8, UnpackError> {
        unpack::unpack_i8(&mut self.inner)
    }

    fn unpack_i16(&mut self) -> Result<i16, UnpackError> {
        unpack::unpack_i16(&mut self.inner)
    }

    fn unpack_i32(&mut self) -> Result<i32, UnpackError> {
        unpack::unpack_i32(&mut self.inner)
    }

    fn unpack_i64(&mut self) -> Result<i64, UnpackError> {
        unpack::unpack_i64(&mut self.inner)
    }

    fn unpack_nil<T>(&mut self) -> Result<Option<T>, UnpackError> {
        unpack::unpack_nil(&mut self.inner)
    }

    fn unpack_bool(&mut self) -> Result<bool, UnpackError> {
        unpack::unpack_bool(&mut self.inner)
    }

    fn unpack_string(&mut self) -> Result<String, UnpackError> {
        unpack::unpack_str(&mut self.inner)
    }

    fn unpack_str_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_str_header(&mut self.inner)
    }

    fn unpack_array_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_array_header(&mut self.inner)
    }

    fn unpack_map_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_map_header(&mut self.inner)
    }

    fn unpack_bin_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_bin_header(&mut self.inner)
    }

    fn unpack_fixext1(&mut self) -> Result<(i8, u8), UnpackError> {
        unpack::unpack_fixext1(&mut self.inner)
    }

    fn unpack_fixext2(&mut self) -> Result<(i8, [u8; 2]), UnpackError> {
        unpack::unpack_fixext2(&mut self.inner)
    }

    fn unpack_fixext4(&mut self) -> Result<(i8, [u8; 4]), UnpackError> {
        unpack::unpack_fixext4(&mut self.inner)
    }

    fn unpack_fixext8(&mut self) -> Result<(i8, [u8; 8]), UnpackError> {
        unpack::unpack_fixext8(&mut self.inner)
    }

    fn unpack_fixext16(&mut self) -> Result<(i8, [u8; 16]), UnpackError> {
        unpack::unpack_fixext16(&mut self.inner)
    }
}
