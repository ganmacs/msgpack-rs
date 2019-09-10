use crate::{unpack, value::RefValue, BufferedRead, UnpackError, Value};

use std::iter::Iterator;
use std::{cmp, io};
pub struct RefUnpacker;
pub struct RefUnpackFeeder<'a, R>(&'a mut R);
use bytes::{Buf, BytesMut};

impl<'a, R> Iterator for RefUnpackFeeder<'a, R>
where
    R: BufferedRead<'a>,
{
    type Item = RefValue<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        RefUnpacker::unpack_ref_value(self.0).ok()
    }
}

impl RefUnpacker {
    pub fn feed<'a, R>(rd: &'a mut R) -> RefUnpackFeeder<'a, R>
    where
        R: BufferedRead<'a>,
    {
        RefUnpackFeeder(rd)
    }

    pub fn unpack_ref_value<'a, R>(rd: &mut R) -> Result<RefValue<'a>, UnpackError>
    where
        R: BufferedRead<'a>,
    {
        unpack::ref_value::unpack_value_ref(rd)
    }
}

#[derive(Debug)]
pub struct Unpacker<R> {
    inner: R,
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

impl<T> Unpacker<T>
where
    T: io::Read,
{
    pub fn from_reader(inner: T) -> Self {
        Unpacker { inner }
    }

    pub fn iter(&mut self) -> UnpackerIter<&mut Unpacker<T>> {
        UnpackerIter::new(self)
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

#[derive(Debug)]
pub struct UnpackerIter<R>(R);

impl<R> UnpackerIter<R> {
    pub fn new(inner: R) -> Self {
        UnpackerIter(inner)
    }
}

impl<'a, R> Iterator for UnpackerIter<R>
where
    R: io::Read,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        unpack::unpack_value(&mut self.0).ok()
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

impl<R: io::Read> Unpacker<R> {
    pub fn unpack_u8(&mut self) -> Result<u8, UnpackError> {
        unpack::unpack_u8(&mut self.inner)
    }

    pub fn unpack_u16(&mut self) -> Result<u16, UnpackError> {
        unpack::unpack_u16(&mut self.inner)
    }

    pub fn unpack_u32(&mut self) -> Result<u32, UnpackError> {
        unpack::unpack_u32(&mut self.inner)
    }

    pub fn unpack_u64(&mut self) -> Result<u64, UnpackError> {
        unpack::unpack_u64(&mut self.inner)
    }

    pub fn unpack_i8(&mut self) -> Result<i8, UnpackError> {
        unpack::unpack_i8(&mut self.inner)
    }

    pub fn unpack_i16(&mut self) -> Result<i16, UnpackError> {
        unpack::unpack_i16(&mut self.inner)
    }

    pub fn unpack_i32(&mut self) -> Result<i32, UnpackError> {
        unpack::unpack_i32(&mut self.inner)
    }

    pub fn unpack_i64(&mut self) -> Result<i64, UnpackError> {
        unpack::unpack_i64(&mut self.inner)
    }

    pub fn unpack_nil<T>(&mut self) -> Result<Option<T>, UnpackError> {
        unpack::unpack_nil(&mut self.inner)
    }

    pub fn unpack_bool(&mut self) -> Result<bool, UnpackError> {
        unpack::unpack_bool(&mut self.inner)
    }

    pub fn unpack_string(&mut self) -> Result<String, UnpackError> {
        unpack::unpack_str(&mut self.inner)
    }

    pub fn unpack_str_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_str_header(&mut self.inner)
    }

    pub fn unpack_ary_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_ary_header(&mut self.inner)
    }

    pub fn unpack_map_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_map_header(&mut self.inner)
    }

    pub fn unpack_bin_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_bin_header(&mut self.inner)
    }

    pub fn unpack_fixext1(&mut self) -> Result<(i8, u8), UnpackError> {
        unpack::unpack_fixext1(&mut self.inner)
    }

    pub fn unpack_fixext2(&mut self) -> Result<(i8, [u8; 2]), UnpackError> {
        unpack::unpack_fixext2(&mut self.inner)
    }

    pub fn unpack_fixext4(&mut self) -> Result<(i8, [u8; 4]), UnpackError> {
        unpack::unpack_fixext4(&mut self.inner)
    }

    pub fn unpack_fixext8(&mut self) -> Result<(i8, [u8; 8]), UnpackError> {
        unpack::unpack_fixext8(&mut self.inner)
    }

    pub fn unpack_fixext16(&mut self) -> Result<(i8, [u8; 16]), UnpackError> {
        unpack::unpack_fixext16(&mut self.inner)
    }

    pub fn unpack_value(&mut self) -> Result<Value, UnpackError> {
        unpack::unpack_value(&mut self.inner)
    }
}
