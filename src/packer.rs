use std::io;

use crate::{pack, PackError, Value};

#[derive(Debug)]
pub struct Packer<W> {
    pub(crate) wr: W,
}

impl<W> Packer<W> {
    pub fn new(writer: W) -> Packer<W> {
        Packer { wr: writer }
    }

    pub fn get_ref(&self) -> &W {
        &self.wr
    }

    pub fn get_mut(&mut self) -> &mut W {
        &mut self.wr
    }

    pub fn into_inner(self) -> W {
        self.wr
    }
}

impl<W: io::Write> Packer<W> {
    pub fn pack_nil(&mut self) -> Result<(), PackError> {
        pack::pack_nil(&mut self.wr)
    }

    pub fn pack_uint(&mut self, v: u64) -> Result<(), PackError> {
        pack::pack_from_u64(&mut self.wr, v)
    }
    pub fn pack_from_u8(&mut self, v: u8) -> Result<(), PackError> {
        pack::pack_from_u8(&mut self.wr, v)
    }

    pub fn pack_from_u16(&mut self, v: u16) -> Result<(), PackError> {
        pack::pack_from_u16(&mut self.wr, v)
    }

    pub fn pack_from_u32(&mut self, v: u32) -> Result<(), PackError> {
        pack::pack_from_u32(&mut self.wr, v)
    }

    pub fn pack_from_u64(&mut self, v: u64) -> Result<(), PackError> {
        pack::pack_from_u64(&mut self.wr, v)
    }

    pub fn pack_int(&mut self, v: i64) -> Result<(), PackError> {
        pack::pack_from_i64(&mut self.wr, v)
    }

    pub fn pack_from_i8(&mut self, v: i8) -> Result<(), PackError> {
        pack::pack_from_i8(&mut self.wr, v)
    }

    pub fn pack_from_i16(&mut self, v: i16) -> Result<(), PackError> {
        pack::pack_from_i16(&mut self.wr, v)
    }

    pub fn pack_from_i32(&mut self, v: i32) -> Result<(), PackError> {
        pack::pack_from_i32(&mut self.wr, v)
    }

    pub fn pack_from_i64(&mut self, v: i64) -> Result<(), PackError> {
        pack::pack_from_i64(&mut self.wr, v)
    }

    pub fn pack_f32(&mut self, v: f32) -> Result<(), PackError> {
        pack::pack_f32(&mut self.wr, v)
    }

    pub fn pack_f64(&mut self, v: f64) -> Result<(), PackError> {
        pack::pack_f64(&mut self.wr, v)
    }

    pub fn pack_bool(&mut self, v: bool) -> Result<(), PackError> {
        pack::pack_bool(&mut self.wr, v)
    }

    pub fn pack_str(&mut self, v: &str) -> Result<(), PackError> {
        pack::pack_str(&mut self.wr, v)
    }

    pub fn pack_bin(&mut self, v: &[u8]) -> Result<(), PackError> {
        pack::pack_bin(&mut self.wr, v)
    }

    pub fn pack_array_header(&mut self, len: usize) -> Result<(), PackError> {
        pack::pack_array_header(&mut self.wr, len)
    }

    pub fn pack_map_header(&mut self, len: usize) -> Result<(), PackError> {
        pack::pack_map_header(&mut self.wr, len)
    }

    pub fn pack_ext_header(&mut self, ext_type: i8, len: usize) -> Result<(), PackError> {
        pack::pack_ext_header(&mut self.wr, ext_type, len)
    }

    pub fn pack_value(&mut self, v: Value) -> Result<(), PackError> {
        pack::pack_value(&mut self.wr, v)
    }

    pub fn write_payload(&mut self, v: &[u8]) -> Result<(), PackError> {
        pack::write_all(&mut self.wr, v)
    }

    pub fn flush(&mut self) -> Result<(), PackError> {
        self.wr.flush().map_err(PackError::WriteError)
    }
}
