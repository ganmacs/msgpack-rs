mod error;

pub use error::DeError;
use msgpack::{code, pack, unpack};

use serde;
use serde::de::{self, Deserialize, DeserializeOwned, DeserializeSeed, Visitor};
use serde::forward_to_deserialize_any;
use std::io;

#[derive(Clone)]
enum ESize {
    Var4(u32),
    Var2(u16),
    Var1(u8),
}

struct PeekReader<R> {
    code: Option<code::Code>,
    len: Option<ESize>,
    reader: R,
}

impl<R: io::Read> PeekReader<R> {
    pub fn peek_code(&mut self) -> Result<&code::Code, msgpack::UnpackError> {
        if let Some(ref v) = self.code {
            Ok(v)
        } else {
            let code = unpack::read_code(&mut self.reader)?;
            self.code = Some(code);
            Ok(self.code.as_ref().unwrap())
        }
    }

    pub fn peek_size(&mut self) -> Result<ESize, msgpack::UnpackError> {
        if let Some(ref v) = self.len {
            return Ok(v.clone());
        }

        let ret = if let Some(ref v) = self.code {
            v
        } else {
            self.peek_code()?
        };

        use code::CodeSize;
        let esize = match ret.body_size() {
            CodeSize::Var1 => ESize::Var1(unpack::read_data_u8(&mut self.reader)?),
            CodeSize::Var2 => ESize::Var2(unpack::read_data_u16(&mut self.reader)?),
            CodeSize::Var4 => ESize::Var4(unpack::read_data_u32(&mut self.reader)?),
            CodeSize::FixLen(_) => unreachable!(),
        };

        self.len = Some(esize);
        Ok(self.len.clone().unwrap())
    }

    pub fn consume_code(&mut self) -> Option<code::Code> {
        self.code.take()
    }
}

impl<R: io::Read> io::Read for PeekReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(ref v) = self.code.take() {
            buf[0] = u8::from(v);
            let buf_len = buf.len();

            if buf_len <= 1 {
                return Ok(1);
            }

            if let Some(ref es) = self.len.take() {
                match es {
                    ESize::Var1(v) => {
                        if buf_len <= 5 {
                            unreachable!()
                        } else {
                            use pack::primitive::write_data_u8;
                            let mut cur = io::Cursor::new(&mut buf[0..4]);
                            write_data_u8(&mut cur, *v);
                        }
                    } // ESize::Var2(v) =>
                    // ESize::Var4(v) =>
                    _ => unreachable!(),
                }

                self.reader.read(&mut buf[4..])
            } else {
                self.reader.read(&mut buf[1..])
            }

        // if buf.len()  1 {

        // if let Some(ref es) = self.code {
        //     match es {
        //         ESize::Var1(v) => {

        //             buf[]
        //         }
        //         ESize::Var2(v) =>
        //         ESize::Var4(v) =>
        //     }
        // } else {
        //
        //     if buf.len() > 1 {
        //         self.reader.read(&mut buf[1..])
        //     } else {
        //         Ok(1)
        //     }
        // }
        } else {
            self.reader.read(buf)
        }
    }
}

impl<'a, R: msgpack::BufferedRead<'a>> msgpack::BufferedRead<'a> for PeekReader<R> {
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
    R: msgpack::BufferedRead<'de> + 'a,
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
    R: msgpack::BufferedRead<'de> + 'a,
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
                len: None,
                reader: r,
            },
        }
    }
}

impl<'de, R> Deserializer<R>
where
    R: msgpack::BufferedRead<'de>,
{
    fn read_ext<V>(&mut self, len: u32, visitor: V) -> Result<V::Value, DeError>
    where
        V: de::Visitor<'de>,
    {
        let ext_de = ExtDeserializer {
            reader: &mut self.reader,
            len: len as usize,
            rest: 10, //
        };
        visitor.visit_seq(ext_de)
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

impl<'de, 'a, R> serde::Deserializer<'de> for &mut Deserializer<R>
where
    R: msgpack::BufferedRead<'de>,
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

        println!("{:?}", "deserialize_any");

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
            Code::FixExt1 => self.read_ext(1, visitor),
            Code::FixExt2 => self.read_ext(2, visitor),
            Code::FixExt4 => self.read_ext(4, visitor),
            Code::FixExt8 => self.read_ext(8, visitor),
            Code::FixExt16 => self.read_ext(16, visitor),
            Code::Ext8 => {
                let s = self.reader.peek_size()?;
                match s {
                    ESize::Var1(v) => self.read_ext(v as u32, visitor),
                    _ => unreachable!(),
                }
            }
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
        visitor.visit_none() // TODO
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        use code::Code;
        match self.reader.peek_code()? {
            Code::FixExt1
            | Code::FixExt2
            | Code::FixExt4
            | Code::FixExt8
            | Code::FixExt16
            | Code::Ext8
            | Code::Ext16
            | Code::Ext32 => self.deserialize_any(visitor),
            _ => visitor.visit_newtype_struct(self),
        }
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

#[derive(Debug)]
struct ExtDeserializer<'a, R: 'a> {
    reader: &'a mut R,
    len: usize,
    rest: usize,
}

impl<'de, 'a, R> serde::de::SeqAccess<'de> for ExtDeserializer<'a, R>
where
    R: msgpack::BufferedRead<'de> + 'a,
{
    type Error = DeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.rest > 0 {
            self.rest -= 1;
            Ok(Some(seed.deserialize(self)?))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.rest)
    }
}

impl<'de, 'a, R> serde::Deserializer<'de> for &mut ExtDeserializer<'a, R>
where
    R: msgpack::BufferedRead<'de>,
{
    type Error = DeError;

    impl_nums!(deserialize_u8, visit_u8, read_data_u8);
    impl_nums!(deserialize_u16, visit_u16, read_data_u16);
    impl_nums!(deserialize_u32, visit_u32, read_data_u32);
    impl_nums!(deserialize_i8, visit_i8, read_data_i8);
    // impl_nums!(deserialize_i16, visit_i16, read_data_i16);
    // impl_nums!(deserialize_i32, visit_i32, read_data_i32);
    // impl_nums!(deserialize_i64, visit_i64, read_data_i64);
    // impl_nums!(deserialize_u64, visit_u64, read_data_u64);

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(self) // size(u8) + id(i8) + data([u8])
    }

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(10)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let body = unpack::unpack_data_ref(self.reader, self.len)?;
        visitor.visit_borrowed_bytes(body)
    }

    forward_to_deserialize_any! {
        bool u64 i16 i32 i64 f32 f64 char str string unit option
            byte_buf map unit_struct newtype_struct
            tuple_struct struct identifier tuple enum ignored_any
    }
}
