use crate::{unpack, UnpackError, Value};
use bytes;
use std::io;
use std::iter::Iterator;

pub struct Unpacker<R> {
    rd: R,
}

impl<R> Iterator for Unpacker<R>
where
    R: io::Read,
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: identify EOF
        self.unpack_value().ok()
    }
}

// impl<'a, R> Iterator for &'a Unpacker<R>
// where
//     R: io::Read,
// {
//     type Item = &'a Value;

//     fn next(&mut self) -> Option<Self::Item> {
//         // TODO: identify EOF
//         // self.unpack_value().ok().map(|v| v.as_ref())
//     }
// }

impl<R> Unpacker<R> {
    pub fn new(rd: R) -> Self {
        Unpacker { rd }
    }
}

impl<R: io::Read> Unpacker<R> {
    pub fn unpack_u8(&mut self) -> Result<u8, UnpackError> {
        unpack::unpack_u8(&mut self.rd)
    }

    pub fn unpack_u16(&mut self) -> Result<u16, UnpackError> {
        unpack::unpack_u16(&mut self.rd)
    }

    pub fn unpack_u32(&mut self) -> Result<u32, UnpackError> {
        unpack::unpack_u32(&mut self.rd)
    }

    pub fn unpack_u64(&mut self) -> Result<u64, UnpackError> {
        unpack::unpack_u64(&mut self.rd)
    }

    pub fn unpack_i8(&mut self) -> Result<i8, UnpackError> {
        unpack::unpack_i8(&mut self.rd)
    }

    pub fn unpack_i16(&mut self) -> Result<i16, UnpackError> {
        unpack::unpack_i16(&mut self.rd)
    }

    pub fn unpack_i32(&mut self) -> Result<i32, UnpackError> {
        unpack::unpack_i32(&mut self.rd)
    }

    pub fn unpack_i64(&mut self) -> Result<i64, UnpackError> {
        unpack::unpack_i64(&mut self.rd)
    }

    pub fn unpack_nil<T>(&mut self) -> Result<Option<T>, UnpackError> {
        unpack::unpack_nil(&mut self.rd)
    }

    pub fn unpack_bool(&mut self) -> Result<bool, UnpackError> {
        unpack::unpack_bool(&mut self.rd)
    }

    pub fn unpack_string(&mut self) -> Result<String, UnpackError> {
        unpack::unpack_str(&mut self.rd)
    }

    pub fn unpack_str_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_str_header(&mut self.rd)
    }

    pub fn unpack_ary_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_ary_header(&mut self.rd)
    }

    pub fn unpack_map_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_map_header(&mut self.rd)
    }

    pub fn unpack_bin_header(&mut self) -> Result<usize, UnpackError> {
        unpack::unpack_bin_header(&mut self.rd)
    }

    pub fn unpack_fixext1(&mut self) -> Result<(i8, u8), UnpackError> {
        unpack::unpack_fixext1(&mut self.rd)
    }

    pub fn unpack_fixext2(&mut self) -> Result<(i8, [u8; 2]), UnpackError> {
        unpack::unpack_fixext2(&mut self.rd)
    }

    pub fn unpack_fixext4(&mut self) -> Result<(i8, [u8; 4]), UnpackError> {
        unpack::unpack_fixext4(&mut self.rd)
    }

    pub fn unpack_fixext8(&mut self) -> Result<(i8, [u8; 8]), UnpackError> {
        unpack::unpack_fixext8(&mut self.rd)
    }

    pub fn unpack_fixext16(&mut self) -> Result<(i8, [u8; 16]), UnpackError> {
        unpack::unpack_fixext16(&mut self.rd)
    }

    pub fn unpack_value(&mut self) -> Result<Value, UnpackError> {
        unpack::unpack_value(&mut self.rd)
    }
}
