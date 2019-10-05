use std::error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum DeError {
    InvalidSize,
    UnpackError(msgpack::UnpackError),
    Custom(String),
}

impl From<msgpack::UnpackError> for DeError {
    fn from(err: msgpack::UnpackError) -> DeError {
        DeError::UnpackError(err)
    }
}

impl serde::de::Error for DeError {
    fn custom<T: Display>(msg: T) -> DeError {
        DeError::Custom(msg.to_string())
    }
}

impl Display for DeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        error::Error::description(self).fmt(f)
    }
}

impl error::Error for DeError {
    fn description(&self) -> &str {
        use DeError::*;

        match *self {
            InvalidSize => "invalid size",
            UnpackError(ref e) => e.description(),
            Custom(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        use DeError::*;

        match *self {
            UnpackError(ref e) => Some(e),
            Custom(_) => None,
            InvalidSize => None,
        }
    }
}
