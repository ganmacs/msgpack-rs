use crate::PackError;
use byteorder::WriteBytesExt;
use std::io;

#[doc(hidden)]
pub fn write_data_u8<W: io::Write>(wr: &mut W, val: u8) -> Result<(), PackError> {
    wr.write_u8(val).map_err(PackError::WriteError)
}

#[doc(hidden)]
pub fn write_data_u16<W: io::Write>(wr: &mut W, val: u16) -> Result<(), PackError> {
    wr.write_u16::<byteorder::BigEndian>(val)
        .map_err(PackError::WriteError)
}

#[doc(hidden)]
pub fn write_data_u32<W: io::Write>(wr: &mut W, val: u32) -> Result<(), PackError> {
    wr.write_u32::<byteorder::BigEndian>(val)
        .map_err(PackError::WriteError)
}

#[doc(hidden)]
pub fn write_data_u64<W: io::Write>(wr: &mut W, val: u64) -> Result<(), PackError> {
    wr.write_u64::<byteorder::BigEndian>(val)
        .map_err(PackError::WriteError)
}

#[doc(hidden)]
pub fn write_data_i8<W: io::Write>(wr: &mut W, val: i8) -> Result<(), PackError> {
    wr.write_i8(val).map_err(PackError::WriteError)
}

#[doc(hidden)]
pub fn write_data_i16<W: io::Write>(wr: &mut W, val: i16) -> Result<(), PackError> {
    wr.write_i16::<byteorder::BigEndian>(val)
        .map_err(PackError::WriteError)
}

#[doc(hidden)]
pub fn write_data_i32<W: io::Write>(wr: &mut W, val: i32) -> Result<(), PackError> {
    wr.write_i32::<byteorder::BigEndian>(val)
        .map_err(PackError::WriteError)
}

#[doc(hidden)]
pub fn write_data_i64<W: io::Write>(wr: &mut W, val: i64) -> Result<(), PackError> {
    wr.write_i64::<byteorder::BigEndian>(val)
        .map_err(PackError::WriteError)
}

#[doc(hidden)]
pub fn write_data_f32<W: io::Write>(wr: &mut W, val: f32) -> Result<(), PackError> {
    wr.write_f32::<byteorder::BigEndian>(val)
        .map_err(PackError::WriteError)
}

#[doc(hidden)]
pub fn write_data_f64<W: io::Write>(wr: &mut W, val: f64) -> Result<(), PackError> {
    wr.write_f64::<byteorder::BigEndian>(val)
        .map_err(PackError::WriteError)
}

#[doc(hidden)]
pub fn write_all<W: io::Write>(wr: &mut W, v: &[u8]) -> Result<(), PackError> {
    wr.write_all(v).map_err(PackError::WriteError)
}
