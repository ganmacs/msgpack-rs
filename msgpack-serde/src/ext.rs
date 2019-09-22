use bytes;
use msgpack::{self, pack};
use serde::ser::{self, SerializeTupleStruct};

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

pub struct Timestamp(i64, u32);

impl ser::Serialize for Timestamp {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        Ext::from(self).serialize(serializer)
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
    use serde::Serialize;

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
