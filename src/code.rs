// value of code
pub const POS_INT: u8 = 0x00;
pub const NIL: u8 = 0xc0;
pub const FALSE: u8 = 0xc2;
pub const TRUE: u8 = 0xc3;
pub const UINT8: u8 = 0xcc;
pub const UINT16: u8 = 0xcd;
pub const UINT32: u8 = 0xce;
pub const UINT64: u8 = 0xcf;
pub const INT8: u8 = 0xd0;
pub const INT16: u8 = 0xd1;
pub const INT32: u8 = 0xd2;
pub const INT64: u8 = 0xd3;
pub const FLOAT32: u8 = 0xca;
pub const FLOAT64: u8 = 0xcb;
pub const FIX_STR: u8 = 0xa0;
pub const STR8: u8 = 0xd9;
pub const STR16: u8 = 0xda;
pub const STR32: u8 = 0xdb;
pub const BIN8: u8 = 0xc4;
pub const BIN16: u8 = 0xc5;
pub const BIN32: u8 = 0xc6;
pub const FIX_ARRAY: u8 = 0x90;
pub const ARRAY16: u8 = 0xdc;
pub const ARRAY32: u8 = 0xdd;
pub const NEG_INT: u8 = 0xe0;
pub const FIX_MAP: u8 = 0x80;
pub const MAP16: u8 = 0xde;
pub const MAP32: u8 = 0xdf;
pub const FIXEXT1: u8 = 0xd4;
pub const FIXEXT2: u8 = 0xd5;
pub const FIXEXT4: u8 = 0xd6;
pub const FIXEXT8: u8 = 0xd7;
pub const FIXEXT16: u8 = 0xd8;
pub const EXT8: u8 = 0xc7;
pub const EXT16: u8 = 0xc8;
pub const EXT32: u8 = 0xc9;

#[derive(Debug)]
pub enum Code {
    Nil,
    True,
    False,
    PosInt(u8),
    NegInt(i8),
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    FixStr(u8),
    Str8,
    Str16,
    Str32,
    Bin8,
    Bin16,
    Bin32,
    FixArray(u8),
    Array16,
    Array32,
    FixMap(u8),
    Map16,
    Map32,
    FixExt1,
    FixExt2,
    FixExt4,
    FixExt8,
    FixExt16,
    Ext8,
    Ext16,
    Ext32,
    Reserved,
}

impl Code {
    pub fn to_u8(&self) -> u8 {
        match self {
            Code::Nil => NIL,
            Code::True => TRUE,
            Code::False => FALSE,
            Code::PosInt(v) => POS_INT + v,
            Code::NegInt(v) => NEG_INT + (*v as u8),
            Code::Uint8 => UINT8,
            Code::Uint16 => UINT16,
            Code::Uint32 => UINT32,
            Code::Uint64 => UINT64,
            Code::Int8 => INT8,
            Code::Int16 => INT16,
            Code::Int32 => INT32,
            Code::Int64 => INT64,
            Code::Float32 => FLOAT32,
            Code::Float64 => FLOAT64,
            Code::FixStr(v) => FIX_STR + v,
            Code::Str8 => STR8,
            Code::Str16 => STR16,
            Code::Str32 => STR32,
            Code::Bin8 => BIN8,
            Code::Bin16 => BIN16,
            Code::Bin32 => BIN32,
            Code::FixArray(v) => FIX_ARRAY + v,
            Code::Array16 => ARRAY16,
            Code::Array32 => ARRAY32,
            Code::FixMap(v) => FIX_MAP + v,
            Code::Map16 => MAP16,
            Code::Map32 => MAP32,
            Code::FixExt1 => FIXEXT1,
            Code::FixExt2 => FIXEXT2,
            Code::FixExt4 => FIXEXT4,
            Code::FixExt8 => FIXEXT8,
            Code::FixExt16 => FIXEXT16,
            Code::Ext8 => EXT8,
            Code::Ext16 => EXT16,
            Code::Ext32 => EXT32,
            Code::Reserved => unreachable!(), // tmp
        }
    }

    fn from_u8(v: u8) -> Self {
        match v {
            0x00..=0x7f => Code::PosInt(v),
            0x80..=0x8f => Code::FixMap(v - 0x80),
            0x90..=0x9f => Code::FixArray(v - 0x90),
            0xa0..=0xbf => Code::FixStr(v - 0xa0),
            0xe0..=0xff => Code::NegInt(v as i8),
            NIL => Code::Nil,
            FALSE => Code::False,
            TRUE => Code::True,
            UINT8 => Code::Uint8,
            UINT16 => Code::Uint16,
            UINT32 => Code::Uint32,
            UINT64 => Code::Uint64,
            INT8 => Code::Int8,
            INT16 => Code::Int16,
            INT32 => Code::Int32,
            INT64 => Code::Int64,
            FLOAT32 => Code::Float32,
            FLOAT64 => Code::Float64,
            STR8 => Code::Str8,
            STR16 => Code::Str16,
            STR32 => Code::Str32,
            BIN8 => Code::Bin8,
            BIN16 => Code::Bin16,
            BIN32 => Code::Bin32,
            ARRAY16 => Code::Array16,
            ARRAY32 => Code::Array32,
            MAP16 => Code::Map16,
            MAP32 => Code::Map32,
            FIXEXT1 => Code::FixExt1,
            FIXEXT2 => Code::FixExt2,
            FIXEXT4 => Code::FixExt4,
            FIXEXT8 => Code::FixExt8,
            FIXEXT16 => Code::FixExt16,
            EXT8 => Code::Ext8,
            EXT16 => Code::Ext16,
            EXT32 => Code::Ext32,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Code {
    fn from(v: u8) -> Self {
        Code::from_u8(v)
    }
}
