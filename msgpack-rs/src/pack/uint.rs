use super::primitive::*;
use crate::{code, PackError};
use std::io;

const UINT8_MIN: u8 = 128; // 1 << 7
const UINT16_MIN: u16 = 256; // 1 << 8
const UINT32_MIN: u32 = 65536; // 1 << 16
const UINT64_MIN: u64 = 4294967296; // 1 << 32

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
