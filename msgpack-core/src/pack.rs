pub mod primitive;
mod timestamp;

use std::io;

use crate::{code, PackError};
use primitive::*;

pub use primitive::write_all;
pub use timestamp::*;

const UINT8_MIN: u8 = 128; // 1 << 7
const UINT16_MIN: u16 = 256; // 1 << 8
const UINT32_MIN: u32 = 65536; // 1 << 16
const UINT64_MIN: u64 = 4294967296; // 1 << 32
const FIX_INT_MAX: i8 = -1;
const INT8_MAX: i8 = -33;
const INT16_MAX: i16 = -129;
const INT32_MAX: i32 = -32769;
const INT64_MAX: i64 = -2147483649; // -((1 << 31) + 1)
const FIXSTR_LIMIT: usize = 1 << 5;
const STR8_LIMIT: usize = 1 << 8;
const STR16_LIMIT: usize = 1 << 16;
const STR32_LIMIT: usize = 1 << 32;
const BIN8_LIMIT: usize = 1 << 8;
const BIN16_LIMIT: usize = 1 << 16;
const BIN32_LIMIT: usize = 1 << 32;
const FIXMAP_LIMIT: usize = 1 << 4;
const MAP16_LIMIT: usize = 1 << 16;
const MAP32_LIMIT: usize = 1 << 32;

pub fn pack_pos_fixint<W: io::Write>(writer: &mut W, v: u8) -> Result<(), PackError> {
    if v < UINT8_MIN {
        write_data_u8(writer, code::POS_INT | v)
    } else {
        use std::io::ErrorKind;
        Err(PackError::WriteError(io::Error::new(
            ErrorKind::Other,
            format!("Value is out of range for positive fix int: {:?}", v),
        )))
    }
}

pub fn pack_u8<W: io::Write>(writer: &mut W, v: u8) -> Result<(), PackError> {
    write_data_u8(writer, code::UINT8)?;
    write_data_u8(writer, v)
}

pub fn pack_u16<W: io::Write>(writer: &mut W, v: u16) -> Result<(), PackError> {
    write_data_u8(writer, code::UINT16)?;
    write_data_u16(writer, v)
}

pub fn pack_u32<W: io::Write>(writer: &mut W, v: u32) -> Result<(), PackError> {
    write_data_u8(writer, code::UINT32)?;
    write_data_u32(writer, v)
}

pub fn pack_u64<W: io::Write>(writer: &mut W, v: u64) -> Result<(), PackError> {
    write_data_u8(writer, code::UINT64)?;
    write_data_u64(writer, v)
}

pub fn pack_from_u8<W: io::Write>(writer: &mut W, v: u8) -> Result<(), PackError> {
    if v >= UINT8_MIN {
        pack_u8(writer, v)
    } else {
        pack_pos_fixint(writer, v)
    }
}

pub fn pack_from_u16<W: io::Write>(writer: &mut W, v: u16) -> Result<(), PackError> {
    if v >= UINT16_MIN {
        pack_u16(writer, v)
    } else {
        pack_from_u8(writer, v as u8)
    }
}

pub fn pack_from_u32<W: io::Write>(writer: &mut W, v: u32) -> Result<(), PackError> {
    if v >= UINT32_MIN {
        pack_u32(writer, v)
    } else {
        pack_from_u16(writer, v as u16)
    }
}

pub fn pack_from_u64<W: io::Write>(writer: &mut W, v: u64) -> Result<(), PackError> {
    if v >= UINT64_MIN {
        pack_u64(writer, v)
    } else {
        pack_from_u32(writer, v as u32)
    }
}

pub fn pack_neg_fix_int<W: io::Write>(writer: &mut W, v: i8) -> Result<(), PackError> {
    if v > INT8_MAX && v <= FIX_INT_MAX {
        write_data_i8(writer, v)
    } else {
        use std::io::ErrorKind;
        Err(PackError::WriteError(io::Error::new(
            ErrorKind::Other,
            format!("Value is out of range for negative fix int : {:?}", v),
        )))
    }
}

pub fn pack_i8<W: io::Write>(writer: &mut W, v: i8) -> Result<(), PackError> {
    write_data_u8(writer, code::INT8)?;
    write_data_i8(writer, v)
}

pub fn pack_i16<W: io::Write>(writer: &mut W, v: i16) -> Result<(), PackError> {
    write_data_u8(writer, code::INT16)?;
    write_data_i16(writer, v)
}

pub fn pack_i32<W: io::Write>(writer: &mut W, v: i32) -> Result<(), PackError> {
    write_data_u8(writer, code::INT32)?;
    write_data_i32(writer, v)
}

pub fn pack_i64<W: io::Write>(writer: &mut W, v: i64) -> Result<(), PackError> {
    write_data_u8(writer, code::INT64)?;
    write_data_i64(writer, v)
}

pub fn pack_from_i8<W: io::Write>(writer: &mut W, v: i8) -> Result<(), PackError> {
    if v > FIX_INT_MAX {
        write_data_u8(writer, v as u8)
    } else if v > INT8_MAX {
        pack_neg_fix_int(writer, v)
    } else {
        pack_i8(writer, v)
    }
}

pub fn pack_from_i16<W: io::Write>(writer: &mut W, v: i16) -> Result<(), PackError> {
    if v > INT16_MAX {
        pack_from_i8(writer, v as i8)
    } else {
        pack_i16(writer, v)
    }
}

pub fn pack_from_i32<W: io::Write>(writer: &mut W, v: i32) -> Result<(), PackError> {
    if v > INT32_MAX {
        pack_from_i16(writer, v as i16)
    } else {
        pack_i32(writer, v)
    }
}

pub fn pack_from_i64<W: io::Write>(writer: &mut W, v: i64) -> Result<(), PackError> {
    if v > INT64_MAX {
        pack_from_i32(writer, v as i32)
    } else {
        pack_i64(writer, v)
    }
}

pub fn pack_f32<W: io::Write>(writer: &mut W, v: f32) -> Result<(), PackError> {
    write_data_u8(writer, code::FLOAT32)?;
    write_data_f32(writer, v)
}

pub fn pack_f64<W: io::Write>(writer: &mut W, v: f64) -> Result<(), PackError> {
    write_data_u8(writer, code::FLOAT64)?;
    write_data_f64(writer, v)
}

pub fn pack_bool<W: io::Write>(writer: &mut W, v: bool) -> Result<(), PackError> {
    write_data_u8(writer, if v { code::TRUE } else { code::FALSE })
}

pub fn pack_nil<W: io::Write>(writer: &mut W) -> Result<(), PackError> {
    write_data_u8(writer, code::NIL)
}

pub fn pack_str<W: io::Write>(writer: &mut W, v: &str) -> Result<(), PackError> {
    let len = v.len();
    if len < FIXSTR_LIMIT {
        write_data_u8(writer, code::FIX_STR | len as u8)?;
        write_all(writer, v[..len].as_bytes())
    } else if len < STR8_LIMIT {
        write_data_u8(writer, code::STR8)?;
        write_data_u8(writer, len as u8)?;
        write_all(writer, &v[..len].as_bytes())
    } else if len < STR16_LIMIT {
        write_data_u8(writer, code::STR16)?;
        write_data_u16(writer, len as u16)?;
        write_all(writer, v[..len].as_bytes())
    } else if len < STR32_LIMIT {
        write_data_u8(writer, code::STR32)?;
        write_data_u32(writer, len as u32)?;
        write_all(writer, v[..len].as_bytes())
    } else {
        unreachable!()
    }
}

// for string contains invalid byte sequence.
pub fn pack_str_from_slice<W: io::Write>(writer: &mut W, v: &[u8]) -> Result<(), PackError> {
    let len = v.len();
    if len < FIXSTR_LIMIT {
        write_data_u8(writer, code::FIX_STR | len as u8)?;
        write_all(writer, &v[..len])
    } else if len < STR8_LIMIT {
        write_data_u8(writer, code::STR8)?;
        write_data_u8(writer, len as u8)?;
        write_all(writer, &v[..len])
    } else if len < STR16_LIMIT {
        write_data_u8(writer, code::STR16)?;
        write_data_u16(writer, len as u16)?;
        write_all(writer, &v[..len])
    } else if len < STR32_LIMIT {
        write_data_u8(writer, code::STR32)?;
        write_data_u32(writer, len as u32)?;
        write_all(writer, &v[..len])
    } else {
        unreachable!()
    }
}

pub fn pack_bin<W: io::Write>(writer: &mut W, v: &[u8]) -> Result<(), PackError> {
    let len = v.len();
    if len < BIN8_LIMIT {
        write_data_u8(writer, code::BIN8)?;
        write_data_u8(writer, len as u8)?;
        write_all(writer, &v[..len])
    } else if len < BIN16_LIMIT {
        write_data_u8(writer, code::BIN16)?;
        write_data_u16(writer, len as u16)?;
        write_all(writer, &v[..len])
    } else if len < BIN32_LIMIT {
        write_data_u8(writer, code::BIN32)?;
        write_data_u32(writer, len as u32)?;
        write_all(writer, &v[..len])
    } else {
        unreachable!()
    }
}

const FIXARRAY_LIMIT: usize = 1 << 4;
const ARRAY16_LIMIT: usize = 1 << 16;
const ARRAY32_LIMIT: usize = 1 << 32;

pub fn pack_array_header<W: io::Write>(writer: &mut W, len: usize) -> Result<(), PackError> {
    if len < FIXARRAY_LIMIT {
        write_data_u8(writer, code::FIX_ARRAY | (len as u8))
    } else if len < ARRAY16_LIMIT {
        write_data_u8(writer, code::ARRAY16)?;
        write_data_u16(writer, len as u16)
    } else if len < ARRAY32_LIMIT {
        write_data_u8(writer, code::ARRAY32)?;
        write_data_u32(writer, len as u32)
    } else {
        unreachable!()
    }
}

pub fn pack_map_header<W: io::Write>(writer: &mut W, len: usize) -> Result<(), PackError> {
    if len < FIXMAP_LIMIT {
        write_data_u8(writer, code::FIX_MAP | (len as u8))
    } else if len < MAP16_LIMIT {
        write_data_u8(writer, code::MAP16)?;
        write_data_u16(writer, len as u16)
    } else if len < MAP32_LIMIT {
        write_data_u8(writer, code::MAP32)?;
        write_data_u32(writer, len as u32)
    } else {
        unreachable!()
    }
}

pub fn pack_ext_header<W: io::Write>(
    writer: &mut W,
    ext_type: i8,
    len: usize,
) -> Result<(), PackError> {
    use std::io::ErrorKind;
    if ext_type < 0 {
        return Err(PackError::WriteError(io::Error::new(
            ErrorKind::Other,
            "type < 0 is reserved for future extension",
        )));
    }
    match len {
        1 => write_data_u8(writer, code::FIXEXT1)?,
        2 => write_data_u8(writer, code::FIXEXT2)?,
        4 => write_data_u8(writer, code::FIXEXT4)?,
        8 => write_data_u8(writer, code::FIXEXT8)?,
        16 => write_data_u8(writer, code::FIXEXT16)?,
        17..=256 => {
            write_data_u8(writer, code::EXT8)?;
            write_data_u8(writer, len as u8)?
        }
        256..=65536 => {
            write_data_u8(writer, code::EXT16)?;
            write_data_u16(writer, len as u16)?
        }
        _ => {
            write_data_u8(writer, code::EXT32)?;
            write_data_u32(writer, len as u32)?
        }
    };

    write_data_i8(writer, ext_type)
}
