mod int;
mod primitive;
mod timestamp;
mod uint;
mod value;

use crate::{code, PackError};
use primitive::*;
use std::io;

pub use int::*;
pub use primitive::write_all;
pub use timestamp::*;
pub use uint::*;
pub use value::pack_value;

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

const FIXSTR_LIMIT: usize = 1 << 5;
const STR8_LIMIT: usize = 1 << 8;
const STR16_LIMIT: usize = 1 << 16;
const STR32_LIMIT: usize = 1 << 32;

pub fn pack_str<W: io::Write>(writer: &mut W, v: &str) -> Result<(), PackError> {
    let len = v.len();
    if len < FIXSTR_LIMIT {
        write_data_u8(writer, code::FIX_STR | len as u8)?;
        write_all(writer, v[..len].as_bytes())
    } else if len < STR8_LIMIT {
        write_data_u8(writer, code::STR8)?;
        write_data_u8(writer, len as u8)?;
        write_all(writer, v[..len].as_bytes())
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

const BIN8_LIMIT: usize = 1 << 8;
const BIN16_LIMIT: usize = 1 << 16;
const BIN32_LIMIT: usize = 1 << 32;

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

pub fn pack_ary_header<W: io::Write>(writer: &mut W, len: usize) -> Result<(), PackError> {
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

const FIXMAP_LIMIT: usize = 1 << 4;
const MAP16_LIMIT: usize = 1 << 16;
const MAP32_LIMIT: usize = 1 << 32;

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
