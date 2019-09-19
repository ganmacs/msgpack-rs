mod compound;
mod error;

use msgpack::pack;
use std::io;

use compound::Compound;
pub use error::SerError;

#[derive(Debug)]
pub struct Serializer<W> {
    wr: W,
}

impl<W> Serializer<W> {
    pub fn new(wr: W) -> Self {
        Serializer { wr }
    }
}

macro_rules! delegate_impl {
    ($ser_method:ident, $pack_method:ident, $typ:ty) => {
        #[inline]
        fn $ser_method(self, v: $typ) -> Result<Self::Ok, Self::Error> {

            pack::$pack_method(&mut self.wr, v).map_err(Self::Error::from)
        }
    }
}

impl<'a, W> serde::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
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

    delegate_impl!(serialize_i8, pack_from_i8, i8);
    delegate_impl!(serialize_i16, pack_from_i16, i16);
    delegate_impl!(serialize_i32, pack_from_i32, i32);
    delegate_impl!(serialize_i64, pack_from_i64, i64);
    delegate_impl!(serialize_u8, pack_from_u8, u8);
    delegate_impl!(serialize_u16, pack_from_u16, u16);
    delegate_impl!(serialize_u32, pack_from_u32, u32);
    delegate_impl!(serialize_u64, pack_from_u64, u64);
    delegate_impl!(serialize_bool, pack_bool, bool);
    delegate_impl!(serialize_f32, pack_f32, f32);
    delegate_impl!(serialize_f64, pack_f64, f64);
    delegate_impl!(serialize_str, pack_str, &str);
    delegate_impl!(serialize_bytes, pack_bin, &[u8]);

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
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
        pack::pack_array_header(&mut self.wr, 0).map_err(Self::Error::from)
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
                pack::pack_array_header(&mut self.wr, len)?;
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