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
            UnpackError(ref e) => e.description(),
            Custom(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        use DeError::*;

        match *self {
            UnpackError(ref e) => Some(e),
            Custom(_) => None,
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
            buf[0] = v.to_u8();
            self.reader.read(&mut buf[1..])
        } else {
            self.reader.read(buf)
        }
    }
}

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

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_none()
        // let marker = match self.marker.take() {
        //     Some(marker) => marker,
        //     None => rmp::decode::read_marker(&mut self.rd)?,
        // };

        // match marker {
        //     Marker::Null => visitor.visit_unit(),
        //     Marker::True => visitor.visit_bool(true),
        //     Marker::False => visitor.visit_bool(false),
        //     Marker::FixPos(val) => visitor.visit_u8(val),
        //     Marker::FixNeg(val) => visitor.visit_i8(val),
        //     Marker::U8 => visitor.visit_u8(rmp::decode::read_data_u8(&mut self.rd)?),
        //     Marker::U16 => visitor.visit_u16(rmp::decode::read_data_u16(&mut self.rd)?),
        //     Marker::U32 => visitor.visit_u32(rmp::decode::read_data_u32(&mut self.rd)?),
        //     Marker::U64 => visitor.visit_u64(rmp::decode::read_data_u64(&mut self.rd)?),
        //     Marker::I8 => visitor.visit_i8(rmp::decode::read_data_i8(&mut self.rd)?),
        //     Marker::I16 => visitor.visit_i16(rmp::decode::read_data_i16(&mut self.rd)?),
        //     Marker::I32 => visitor.visit_i32(rmp::decode::read_data_i32(&mut self.rd)?),
        //     Marker::I64 => visitor.visit_i64(rmp::decode::read_data_i64(&mut self.rd)?),
        //     Marker::F32 => visitor.visit_f32(rmp::decode::read_data_f32(&mut self.rd)?),
        //     Marker::F64 => visitor.visit_f64(rmp::decode::read_data_f64(&mut self.rd)?),
        //     Marker::FixStr(len) => self.read_str_data(len as u32, visitor),
        //     Marker::Str8 => {
        //         let len = read_u8(&mut self.rd)?;
        //         self.read_str_data(len as u32, visitor)
        //     }
        //     Marker::Str16 => {
        //         let len = read_u16(&mut self.rd)?;
        //         self.read_str_data(len as u32, visitor)
        //     }
        //     Marker::Str32 => {
        //         let len = read_u32(&mut self.rd)?;
        //         self.read_str_data(len as u32, visitor)
        //     }
        //     Marker::FixArray(len) => self.read_array(len as u32, visitor),
        //     Marker::Array16 => {
        //         let len = read_u16(&mut self.rd)?;
        //         self.read_array(len as u32, visitor)
        //     }
        //     Marker::Array32 => {
        //         let len = read_u32(&mut self.rd)?;
        //         self.read_array(len, visitor)
        //     }
        //     Marker::FixMap(len) => self.read_map(len as u32, visitor),
        //     Marker::Map16 => {
        //         let len = read_u16(&mut self.rd)?;
        //         self.read_map(len as u32, visitor)
        //     }
        //     Marker::Map32 => {
        //         let len = read_u32(&mut self.rd)?;
        //         self.read_map(len, visitor)
        //     }
        //     Marker::Bin8 => {
        //         let len = read_u8(&mut self.rd)?;
        //         self.read_bytes(len as u32, visitor)
        //     }
        //     Marker::Bin16 => {
        //         let len = read_u16(&mut self.rd)?;
        //         self.read_bytes(len as u32, visitor)
        //     }
        //     Marker::Bin32 => {
        //         let len = read_u32(&mut self.rd)?;
        //         self.read_bytes(len, visitor)
        //     }
        //     Marker::Reserved => Err(Error::TypeMismatch(Marker::Reserved)),
        //     marker => Err(Error::TypeMismatch(marker)),
        // }
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

    forward_to_deserialize_any! {
         f32 f64 char
        str string bytes byte_buf unit seq map
        tuple_struct struct identifier tuple
        ignored_any
    }
}
