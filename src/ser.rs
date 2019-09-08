mod compound;
mod error;

use compound::Compound;
use std::io::Write;

use crate::{pack, packer};
use error::SerError;

impl<'a, W> serde::Serializer for &'a mut packer::Packer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = SerError;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        pack::pack_bool(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        pack::pack_from_i8(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        pack::pack_from_i16(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        pack::pack_from_i32(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        pack::pack_from_i64(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        pack::pack_from_u8(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        pack::pack_from_u16(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        pack::pack_from_u32(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        pack::pack_from_u64(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        pack::pack_f32(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        pack::pack_f64(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        pack::pack_str(&mut self.wr, v).map_err(Self::Error::from)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        pack::pack_bin(&mut self.wr, value).map_err(Self::Error::from)
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized + serde::Serialize>(self, v: &T) -> Result<(), Self::Error> {
        v.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        pack::pack_nil(&mut self.wr).map_err(Self::Error::from)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        pack::pack_ary_header(&mut self.wr, 0).map_err(Self::Error::from)
    }

    fn serialize_unit_variant(
        self,
        _name: &str,
        _idx: u32,
        variant: &str,
    ) -> Result<Self::Ok, Self::Error> {
        pack::pack_map_header(&mut self.wr, 1)?;
        self.serialize_str(variant).map_err(Self::Error::from)?;
        self.serialize_unit().map_err(Self::Error::from)
    }

    fn serialize_newtype_struct<T: ?Sized + serde::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + serde::Serialize>(
        self,
        _name: &'static str,
        _idx: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        pack::pack_map_header(&mut self.wr, 1)?;
        // TODO
        self.serialize_str(variant)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(len) => {
                pack::pack_ary_header(&mut self.wr, len)?;
                Ok(self.compound())
            }
            None => Err(error::SerError::MustHaveLength),
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _idx: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        pack::pack_map_header(&mut self.wr, 1)?;
        // TODO
        self.serialize_str(variant)?;
        self.serialize_tuple_struct(name, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match len {
            Some(len) => {
                pack::pack_map_header(&mut self.wr, len)?;
                Ok(self.compound())
            }
            None => Err(error::SerError::MustHaveLength),
        }
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        pack::pack_map_header(&mut self.wr, len)?;
        Ok(self.compound())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _id: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        pack::pack_map_header(&mut self.wr, len)?;
        self.serialize_str(variant)?;
        pack::pack_map_header(&mut self.wr, 1)?;
        Ok(self.compound())
    }
}
