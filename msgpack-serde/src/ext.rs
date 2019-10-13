use bytes;
use serde_bytes;

use msgpack::{self, pack, unpack};
use serde::de;
use serde::ser::{self, SerializeTupleStruct};
use std::{fmt, io};

pub const EXT_TOKEN: &'static str = "$serde_json::private::Ext";

#[derive(Debug, PartialEq)]
enum ExtType {
    FixExt1,
    FixExt2,
    FixExt4,
    FixExt8,
    FixExt16,
    Ext8(u8),
    Ext16(u16),
    Ext32(u32),
}

impl<'de> de::Deserialize<'de> for ExtType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ExtTypeVisitor;

        impl<'de> serde::de::Visitor<'de> for ExtTypeVisitor {
            type Value = ExtType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                formatter.write_str("valid msgpack exttype")
            }

            #[inline]
            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                let h = seq.next_element();
                let h: u8 = h?.ok_or_else(|| de::Error::invalid_length(0, &self))?;

                Ok(match h {
                    msgpack::code::FIXEXT1 => ExtType::FixExt1,
                    msgpack::code::FIXEXT2 => ExtType::FixExt2,
                    msgpack::code::FIXEXT4 => ExtType::FixExt4,
                    msgpack::code::FIXEXT8 => ExtType::FixExt8,
                    msgpack::code::FIXEXT16 => ExtType::FixExt16,
                    msgpack::code::EXT8 => {
                        let v: u8 = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        ExtType::Ext8(v)
                    }
                    msgpack::code::EXT16 => {
                        let v: u16 = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        ExtType::Ext16(v)
                    }
                    msgpack::code::EXT32 => {
                        let v: u32 = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                        ExtType::Ext32(v)
                    }
                    v => {
                        return Err(de::Error::custom(format!(
                            "invalid code for ext type: {:}",
                            v
                        )));
                    }
                })
            }
        }

        deserializer.deserialize_seq(ExtTypeVisitor)
    }
}

impl From<&ExtType> for u8 {
    fn from(v: &ExtType) -> Self {
        match v {
            ExtType::FixExt1 => msgpack::code::FIXEXT1,
            ExtType::FixExt2 => msgpack::code::FIXEXT2,
            ExtType::FixExt4 => msgpack::code::FIXEXT4,
            ExtType::FixExt8 => msgpack::code::FIXEXT8,
            ExtType::FixExt16 => msgpack::code::FIXEXT16,
            ExtType::Ext8(_) => msgpack::code::EXT8,
            ExtType::Ext16(_) => msgpack::code::EXT16,
            ExtType::Ext32(_) => msgpack::code::EXT32,
        }
    }
}

#[derive(Debug)]
pub struct Ext {
    ext_type: ExtType,
    typ: i8,
    data: bytes::Bytes, // TODO: fix trait base
}

struct ExtVisitor;

impl<'de> de::Deserialize<'de> for Ext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        impl<'de> serde::de::Visitor<'de> for ExtVisitor {
            type Value = Ext;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                formatter.write_str("valid msgpack")
            }

            #[inline]
            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                println!("visit_seq");
                let ext: ExtType = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                println!("ext: {:?}", ext);
                let id: i8 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                println!("id {:?}", id);

                let data = seq.next_element();
                println!("{:?}", data);
                let data: &serde_bytes::Bytes =
                    data?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                println!("data: {:?}", data);

                Ok(Ext {
                    ext_type: ext,
                    typ: id,
                    data: bytes::Bytes::from(data.as_ref()),
                })
            }
        }

        deserializer.deserialize_newtype_struct("Ext", ExtVisitor)
    }
}

impl Ext {
    pub fn fixext1(typ: i8, data: &[u8]) -> Self {
        Ext {
            ext_type: ExtType::FixExt1,
            data: bytes::Bytes::from(data), // COPY?
            typ,
        }
    }

    pub fn fixext2(typ: i8, data: &[u8]) -> Self {
        Ext {
            ext_type: ExtType::FixExt2,
            data: bytes::Bytes::from(data), // COPY?
            typ,
        }
    }

    pub fn fixext4(typ: i8, data: &[u8]) -> Self {
        Ext {
            ext_type: ExtType::FixExt4,
            data: bytes::Bytes::from(data), // COPY?
            typ,
        }
    }

    pub fn fixext8(typ: i8, data: &[u8]) -> Self {
        Ext {
            ext_type: ExtType::FixExt8,
            data: bytes::Bytes::from(data), // COPY?
            typ,
        }
    }

    pub fn fixext16(typ: i8, data: &[u8]) -> Self {
        Ext {
            ext_type: ExtType::FixExt16,
            data: bytes::Bytes::from(data), // COPY?
            typ,
        }
    }

    pub fn ext8(len: u8, typ: i8, data: &[u8]) -> Self {
        Ext {
            ext_type: ExtType::Ext8(len),
            data: bytes::Bytes::from(data), // COPY?
            typ,
        }
    }

    pub fn ext16(len: u16, typ: i8, data: &[u8]) -> Self {
        Ext {
            ext_type: ExtType::Ext16(len),
            data: bytes::Bytes::from(data), // COPY?
            typ,
        }
    }

    pub fn ext32(len: u32, typ: i8, data: &[u8]) -> Self {
        Ext {
            ext_type: ExtType::Ext32(len),
            data: bytes::Bytes::from(data), // COPY?
            typ,
        }
    }
}

impl ser::Serialize for Ext {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut seri = serializer.serialize_tuple_struct(EXT_TOKEN, 0)?;
        seri.serialize_field(&u8::from(&self.ext_type))?;

        match self.ext_type {
            ExtType::Ext8(v) => {
                seri.serialize_field(&v)?;
            }
            ExtType::Ext16(v) => {
                seri.serialize_field(&v)?;
            }
            ExtType::Ext32(v) => {
                seri.serialize_field(&v)?;
            }
            _ => {
                // nothing
            }
        };
        seri.serialize_field(&(self.typ as i8))?;
        seri.serialize_field(&self.data)?; // TODO
        seri.end()
    }
}

#[derive(Debug, PartialEq)]
pub struct Timestamp(i64, u32);

impl Timestamp {
    pub fn new(sec: i64, nsec: u32) -> Self {
        Timestamp(sec, nsec)
    }
}

impl ser::Serialize for Timestamp {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        Ext::from(self).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let v: Ext = deserializer.deserialize_newtype_struct("Timestamp", ExtVisitor)?;
        if v.typ != -1 {
            return Err(de::Error::custom(format!(
                "type is wrong, expectd -1 but {:?}",
                v.typ
            )));
        }

        match v.ext_type {
            ExtType::FixExt4 => {
                let sec = unpack::primitive::read_data_u32(&mut io::Cursor::new(v.data)).unwrap();
                Ok(Timestamp::new(sec as i64, 0))
            }
            ExtType::FixExt8 => {
                let val = unpack::primitive::read_data_u64(&mut io::Cursor::new(v.data)).unwrap();
                let nsec = val >> 34;
                let sec = val & 0x00000003ffffffff;
                Ok(Timestamp::new(sec as i64, nsec as u32))
            }
            ExtType::Ext8(size) => {
                if size != 12 {
                    return Err(de::Error::custom(format!(
                        "expected 12 as Timestamp size but {:?}",
                        size
                    )));
                };
                let mut mm = io::Cursor::new(v.data);
                let sec = unpack::primitive::read_data_u32(&mut mm).unwrap();
                let nsec = unpack::primitive::read_data_u64(&mut mm).unwrap();
                Ok(Timestamp::new(sec as i64, nsec as u32))
            }
            v => {
                return Err(de::Error::custom(format!(
                    "expected FixExt4, FixExt8 or Ext8, actual value {:?}",
                    v
                )))
            }
        }
    }
}

impl<'a> From<&Timestamp> for Ext {
    fn from(v: &Timestamp) -> Self {
        let sec = v.0;
        let nsec = v.1;
        let mut v = vec![]; // TODO

        if sec >> 34 == 0 {
            if nsec == 0 {
                pack::primitive::write_data_u32(&mut v, sec as u32)
                    .expect("expected u32 as timestamp sec ");
                Ext::fixext4(-1, &v)
            } else {
                // TODO: range
                let payload: u64 = ((nsec as u64) << 34) | sec as u64;
                pack::primitive::write_data_u64(&mut v, payload).expect("expect u64");
                Ext::fixext8(-1, &v)
            }
        } else {
            pack::primitive::write_data_u32(&mut v, nsec).expect("u32");
            pack::primitive::write_data_i64(&mut v, sec).expect("u64");
            Ext::ext8(12, -1, &v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[test]
    fn ext8_deserialize() {
        let buf = &[0xd7, 0xff, 0x07, 0xff, 0xff, 0xfc, 0x5d, 0x87, 0x3d, 0x44];
        let mut de = crate::de::Deserializer::new(buf.as_ref());
        let v: Ext = Deserialize::deserialize(&mut de).unwrap();

        assert_eq!(ExtType::FixExt8, v.ext_type);
        assert_eq!(-1, v.typ);
        assert_eq!(
            vec![0x07, 0xff, 0xff, 0xfc, 0x5d, 0x87, 0x3d, 0x44],
            v.data.as_ref()
        );
    }

    #[test]
    fn timestamp_deserialize() {
        let buf = &[0xd7, 0xff, 0x07, 0xff, 0xff, 0xfc, 0x5d, 0x87, 0x3d, 0x44];
        let mut de = crate::de::Deserializer::new(buf.as_ref());
        let v: Timestamp = Deserialize::deserialize(&mut de).unwrap();

        assert_eq!(Timestamp(1569144132, 33554431), v);
    }

    #[test]
    fn timestamp_to_ext() {
        let t = Timestamp(1569144132, 33554431);
        let v = Ext::from(&t);

        assert_eq!(ExtType::FixExt8, v.ext_type);
        assert_eq!(-1, v.typ);
        assert_eq!(
            vec![0x07, 0xff, 0xff, 0xfc, 0x5d, 0x87, 0x3d, 0x44],
            v.data.as_ref()
        );
    }

    #[test]
    fn timestamp_serizlize() {
        let mut writer = vec![];
        let t = Timestamp(1569144132, 33554431);
        let _ = t
            .serialize(&mut crate::ser::Serializer::new(&mut writer))
            .unwrap();

        assert_eq!(
            vec![0xd7, 0xff, 0x07, 0xff, 0xff, 0xfc, 0x5d, 0x87, 0x3d, 0x44],
            writer,
        );
    }
}
