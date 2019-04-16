use super::primitive::*;
use crate::{code, PackError};
use std::io;

const FIX_INT_MAX: i8 = -1;
const INT8_MAX: i8 = -33;
const INT16_MAX: i16 = -129;
const INT32_MAX: i32 = -32769;
const INT64_MAX: i64 = -2147483649; // -((1 << 31) + 1)

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
