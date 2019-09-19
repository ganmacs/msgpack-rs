use std::io;

use crate::{unpack, unpack_value_ref, RefValue, Value};
use msgpack::{BufferedRead, InnerBuf, UnpackError};

pub struct RefUnpackFeeder<'a, R>(&'a mut R);

impl<'a, R> Iterator for RefUnpackFeeder<'a, R>
where
    R: BufferedRead<'a>,
{
    type Item = RefValue<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        RefUnpacker::unpack_ref_value(self.0).ok()
    }
}

pub struct RefUnpacker;

impl RefUnpacker {
    pub fn feeder<'a, R>(rd: &'a mut R) -> RefUnpackFeeder<'a, R>
    where
        R: BufferedRead<'a>,
    {
        RefUnpackFeeder(rd)
    }

    pub fn unpack_ref_value<'a, R>(rd: &mut R) -> Result<RefValue<'a>, UnpackError>
    where
        R: BufferedRead<'a>,
    {
        unpack_value_ref(rd)
    }
}

#[derive(Debug)]
pub struct UnpackerIter<R>(R);

impl<R> UnpackerIter<R> {
    pub fn new(inner: R) -> Self {
        UnpackerIter(inner)
    }
}

pub struct ValueUnpacker<R> {
    inner: R,
}

impl<R> ValueUnpacker<R>
where
    R: io::Read,
{
    pub fn from_reader(inner: R) -> Self {
        ValueUnpacker { inner }
    }

    pub fn iter(&mut self) -> UnpackerIter<&mut ValueUnpacker<R>> {
        UnpackerIter::new(self)
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

impl<R: io::Read> ValueUnpacker<R> {
    pub fn unpack_value(&mut self) -> Result<Value, UnpackError> {
        unpack::unpack_value(&mut self.inner)
    }
}

impl ValueUnpacker<InnerBuf> {
    pub fn new() -> Self {
        ValueUnpacker {
            inner: InnerBuf::new(),
        }
    }
}

impl<T> io::Write for ValueUnpacker<T>
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

impl<T> io::Read for ValueUnpacker<T>
where
    T: io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}
