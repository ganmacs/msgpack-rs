use crate::code;
use crate::unpack;
use crate::unpack_error;
use crate::BufferedRead;

use serde;
use serde::de::{self, Deserialize, DeserializeOwned, DeserializeSeed, Visitor};
use serde::forward_to_deserialize_any;
use std::io;

use std::error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum DeError {
    InvalidSize,
    UnpackError(unpack_error::UnpackError),
    Custom(String),
}

impl From<unpack_error::UnpackError> for DeError {
    fn from(err: unpack_error::UnpackError) -> DeError {
        DeError::UnpackError(err)
    }
}

impl Display for DeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for DeError {
    fn description(&self) -> &str {
        use DeError::*;

        match *self {
            InvalidSize => "invalid size",
            UnpackError(ref e) => e.description(),
            Custom(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        use DeError::*;

        match *self {
            UnpackError(ref e) => Some(e),
            Custom(_) => None,
            InvalidSize => None,
        }
    }
}

impl serde::de::Error for DeError {
    fn custom<T: Display>(msg: T) -> DeError {
        DeError::Custom(msg.to_string())
    }
}

struct PeekReader<R> {
    code: Option<code::Code>,
    reader: R,
}

impl<R: io::Read> PeekReader<R> {
    pub fn peek_code(&mut self) -> Result<&code::Code, unpack_error::UnpackError> {
        if let Some(ref v) = self.code {
            Ok(v)
        } else {
            let code = unpack::read_code(&mut self.reader)?;
            self.code = Some(code);
            Ok(self.code.as_ref().unwrap())
        }
    }

    pub fn consume_code(&mut self) -> Option<code::Code> {
        self.code.take()
    }
}

impl<R: io::Read> io::Read for PeekReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(ref v) = self.code {
            buf[0] = u8::from(v);
            if buf.len() > 1 {
                self.reader.read(&mut buf[1..])
            } else {
                Ok(1)
            }
        } else {
            self.reader.read(buf)
        }
    }
}

impl<'a, R: BufferedRead<'a>> BufferedRead<'a> for PeekReader<R> {
    fn fill_buf(&self) -> io::Result<&'a [u8]> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, len: usize) {
        self.reader.consume(len)
    }
}

struct SeqAccess<'a, R: io::Read + 'a> {
    de: &'a mut Deserializer<R>,
    len: usize,
}

impl<'de, 'a, R> serde::de::SeqAccess<'de> for SeqAccess<'a, R>
where
    R: BufferedRead<'de> + 'a,
{
    type Error = DeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            Ok(Some(seed.deserialize(&mut *self.de)?))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct MapAccess<'a, R: 'a> {
    de: &'a mut Deserializer<R>,
    len: usize,
}

impl<'de, 'a, R> de::MapAccess<'de> for MapAccess<'a, R>
where
    R: BufferedRead<'de> + 'a,
{
    type Error = DeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            Ok(Some(seed.deserialize(&mut *self.de)?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        Ok(seed.deserialize(&mut *self.de)?)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

// struct BytesAccess<'a, R: io::Read + 'a> {
//     de: &'a mut Deserializer<R>,
//     len: usize,
// }

// impl<'de, 'a, R> serde::de::SeqAccess<'de> for BytesAccess<'a, R>
// where
//     R: BufferedRead<'de> + 'a,
// {
//     type Error = DeError;

//     fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
//     where
//         T: serde::de::DeserializeSeed<'de>,
//     {
//         if self.len > 0 {
//             self.len -= 1;
//       // seed.deserialize(MapKey { de: &mut *self.de }).map(Some),
//             Ok(Some(seed.deserialize(&mut *self.de)?))
//         } else {
//             Ok(None)
//         }
//     }

//     fn size_hint(&self) -> Option<usize> {
//         Some(self.len)
//     }
// }

pub struct Deserializer<R> {
    reader: PeekReader<R>,
}

impl<R> Deserializer<R> {
    pub fn new(r: R) -> Self {
        Deserializer {
            reader: PeekReader {
                code: None,
                reader: r,
            },
        }
    }
}

macro_rules! impl_nums {
    ($dser_method:ident, $visitor_method:ident, $unpack_method:ident) => {
        #[inline]
        fn $dser_method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            let v = unpack::$unpack_method(&mut self.reader)?;
            visitor.$visitor_method(v)
        }
    }
}

impl<'de, 'a, R> serde::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: BufferedRead<'de>,
{
    type Error = DeError;

    impl_nums!(deserialize_u8, visit_u8, unpack_u8);
    impl_nums!(deserialize_u16, visit_u16, unpack_u16);
    impl_nums!(deserialize_u32, visit_u32, unpack_u32);
    impl_nums!(deserialize_u64, visit_u64, unpack_u64);
    impl_nums!(deserialize_i8, visit_i8, unpack_i8);
    impl_nums!(deserialize_i16, visit_i16, unpack_i16);
    impl_nums!(deserialize_i32, visit_i32, unpack_i32);
    impl_nums!(deserialize_i64, visit_i64, unpack_i64);
    impl_nums!(deserialize_f32, visit_f32, unpack_f32);
    impl_nums!(deserialize_f64, visit_f64, unpack_f64);

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use code::Code;

        match self.reader.peek_code()? {
            Code::Nil => self.deserialize_unit(visitor),
            Code::True | Code::False => self.deserialize_bool(visitor),
            Code::Uint8 | Code::PosInt(_) => self.deserialize_u8(visitor),
            Code::Uint16 => self.deserialize_u16(visitor),
            Code::Uint32 => self.deserialize_u32(visitor),
            Code::Uint64 => self.deserialize_u64(visitor),
            Code::Int8 | Code::NegInt(_) => self.deserialize_i8(visitor),
            Code::Int16 => self.deserialize_i16(visitor),
            Code::Int32 => self.deserialize_i32(visitor),
            Code::Int64 => self.deserialize_i64(visitor),
            Code::Float32 => self.deserialize_f32(visitor),
            Code::Float64 => self.deserialize_f64(visitor),
            Code::FixStr(_) | Code::Str8 | Code::Str16 | Code::Str32 => {
                self.deserialize_string(visitor)
            }
            Code::Bin8 | Code::Bin16 | Code::Bin32 => self.deserialize_bytes(visitor),
            Code::FixArray(_) | Code::Array16 | Code::Array32 => self.deserialize_seq(visitor),
            Code::FixMap(_) | Code::Map16 | Code::Map32 => self.deserialize_map(visitor),
            // Code::FixExt1 => FIXEXT1,
            // Code::FixExt2 => FIXEXT2,
            // Code::FixExt4 => FIXEXT4,
            // Code::FixExt8 => FIXEXT8,
            // Code::FixExt16 => FIXEXT16,
            // Code::Ext8 => EXT8,
            // Code::Ext16 => EXT16,
            // Code::Ext32 => EXT32,
            Code::Reserved => unreachable!(), // tmp
            _ => unreachable!(),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.reader.peek_code()? {
            code::Code::Nil => {
                let _ = self.reader.consume_code();
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &[&str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_none()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match unpack::unpack_bool(&mut self.reader)? {
            true => visitor.visit_bool(true),
            false => visitor.visit_bool(false),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let size = unpack::unpack_array_header(&mut self.reader)?;

        visitor.visit_seq(SeqAccess {
            de: self,
            len: size,
        })
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let size = unpack::unpack_array_header(&mut self.reader)?;
        if size != len {
            return Err(Self::Error::InvalidSize);
        }

        visitor.visit_seq(SeqAccess {
            de: self,
            len: size,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let body = unpack::unpack_str(&mut self.reader)?;
        // TODO: bytes
        visitor.visit_string(body)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let body = unpack::unpack_str_ref(&mut self.reader)?;
        // TODO: bytes_ref
        visitor.visit_str(body)
    }

    fn deserialize_struct<V>(
        self,
        _name: &str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // TODO
        let _size = unpack::unpack_map_header(&mut self.reader)?;
        visitor.visit_map(MapAccess {
            de: self,
            len: fields.len(),
        })
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let size = unpack::unpack_map_header(&mut self.reader)?;
        visitor.visit_map(MapAccess {
            de: self,
            len: size,
        })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let body = unpack::unpack_bin_ref(&mut self.reader)?;
        visitor.visit_bytes(body)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let body = unpack::unpack_bin(&mut self.reader)?;
        visitor.visit_byte_buf(body)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}
