use super::primitive::*;
use crate::{code, PackError};
use std::io;

pub fn pack_timestamp<W: io::Write>(writer: &mut W, sec: i64, nsec: u32) -> Result<(), PackError> {
    if sec >> 34 == 0 {
        if nsec == 0 {
            pack_timestamp32(writer, sec as u32)
        } else {
            pack_timestamp64(writer, sec as u64, nsec)
        }
    } else {
        pack_timestamp96(writer, sec, nsec)
    }
}

pub fn pack_timestamp32<W: io::Write>(writer: &mut W, sec: u32) -> Result<(), PackError> {
    write_data_u8(writer, code::FIXEXT4)?;
    write_data_i8(writer, -1)?;
    write_data_u32(writer, sec)
}

pub fn pack_timestamp64<W: io::Write>(
    writer: &mut W,
    sec: u64,
    nsec: u32,
) -> Result<(), PackError> {
    if sec >> 34 != 0 || nsec >> 30 != 0 {
        use std::io::ErrorKind;
        return Err(PackError::WriteError(io::Error::new(
            ErrorKind::Other,
            "Value is out of range for timestamp64",
        )));
    }
    write_data_u8(writer, code::FIXEXT8)?;
    write_data_i8(writer, -1)?;
    let payload: u64 = ((nsec as u64) << 34) | sec;
    write_data_u64(writer, payload)
}

pub fn pack_timestamp96<W: io::Write>(
    writer: &mut W,
    sec: i64,
    nsec: u32,
) -> Result<(), PackError> {
    write_data_u8(writer, code::EXT8)?;
    write_data_u8(writer, 12)?;
    write_data_i8(writer, -1)?;
    write_data_u32(writer, nsec)?;
    write_data_i64(writer, sec)
}
