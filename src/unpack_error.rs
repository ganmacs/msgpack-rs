use crate::code;
use std::error;
use std::fmt::{self, Display, Formatter};
use std::io::Error;

#[derive(Debug)]
pub enum UnpackError {
    InvalidData(Error),
    TypeMismatch(code::Code),
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
