use crate::{code, UnpackError};
use byteorder::ReadBytesExt;
use std::io;

pub fn read_code<R: io::Read>(rd: &mut R) -> Result<code::Code, UnpackError> {
    read_data_u8(rd).map(code::Code::from)
}

#[doc(hidden)]
pub fn read_data_u8<R: io::Read>(rd: &mut R) -> Result<u8, UnpackError> {
    rd.read_u8().map_err(UnpackError::InvalidData)
}

#[doc(hidden)]
pub fn read_data_u16<R: io::Read>(rd: &mut R) -> Result<u16, UnpackError> {
    rd.read_u16::<byteorder::BigEndian>()
        .map_err(UnpackError::InvalidData)
}

#[doc(hidden)]
pub fn read_data_u32<R: io::Read>(rd: &mut R) -> Result<u32, UnpackError> {
    rd.read_u32::<byteorder::BigEndian>()
        .map_err(UnpackError::InvalidData)
}

#[doc(hidden)]
pub fn read_data_u64<R: io::Read>(rd: &mut R) -> Result<u64, UnpackError> {
    rd.read_u64::<byteorder::BigEndian>()
        .map_err(UnpackError::InvalidData)
}

#[doc(hidden)]
pub fn read_data_i8<R: io::Read>(rd: &mut R) -> Result<i8, UnpackError> {
    rd.read_i8().map_err(UnpackError::InvalidData)
}

#[doc(hidden)]
pub fn read_data_i16<R: io::Read>(rd: &mut R) -> Result<i16, UnpackError> {
    rd.read_i16::<byteorder::BigEndian>()
        .map_err(UnpackError::InvalidData)
}

#[doc(hidden)]
pub fn read_data_i32<R: io::Read>(rd: &mut R) -> Result<i32, UnpackError> {
    rd.read_i32::<byteorder::BigEndian>()
        .map_err(UnpackError::InvalidData)
}

#[doc(hidden)]
pub fn read_data_i64<R: io::Read>(rd: &mut R) -> Result<i64, UnpackError> {
    rd.read_i64::<byteorder::BigEndian>()
        .map_err(UnpackError::InvalidData)
}

#[doc(hidden)]
pub fn read_data_f32<R: io::Read>(rd: &mut R) -> Result<f32, UnpackError> {
    rd.read_f32::<byteorder::BigEndian>()
        .map_err(UnpackError::InvalidData)
}

#[doc(hidden)]
pub fn read_data_f64<R: io::Read>(rd: &mut R) -> Result<f64, UnpackError> {
    rd.read_f64::<byteorder::BigEndian>()
        .map_err(UnpackError::InvalidData)
}

#[doc(hidden)]
pub fn read_data<R: io::Read>(rd: &mut R, buf: &mut [u8]) -> Result<(), UnpackError> {
    rd.read_exact(buf).map_err(UnpackError::InvalidData)
}
