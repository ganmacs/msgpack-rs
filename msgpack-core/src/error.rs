use crate::code;
use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum PackError {
    WriteError(io::Error),
}

impl From<io::Error> for PackError {
    fn from(err: io::Error) -> PackError {
        PackError::WriteError(err)
    }
}

impl Display for PackError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for PackError {
    fn description(&self) -> &str {
        match *self {
            PackError::WriteError(..) => "failed to write data",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PackError::WriteError(ref e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub enum UnpackError {
    InvalidData(io::Error),
    TypeMismatch(code::Code, String),
}

impl Display for UnpackError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for UnpackError {
    fn description(&self) -> &str {
        match *self {
            UnpackError::InvalidData(..) => "failed to read data",
            UnpackError::TypeMismatch(..) => "type isn't match with the expected one",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            UnpackError::InvalidData(ref err) => Some(err),
            UnpackError::TypeMismatch(..) => None,
        }
    }
}
