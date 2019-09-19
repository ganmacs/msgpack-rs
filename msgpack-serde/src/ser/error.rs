use msgpack::PackError;
// use crate::error as pack_error;

use std::error;
use std::fmt::{self, Display};

use serde;

#[derive(Debug)]
pub enum SerError {
    MustHaveLength,
    PackError(PackError),
    Custom(String),
    // InvalidValueWrite(ValueWriteError),
    // UnknownLength,
    // DepthLimitExceeded,
    // Syntax(String),
}

impl From<PackError> for SerError {
    fn from(err: PackError) -> SerError {
        SerError::PackError(err)
    }
}

impl error::Error for SerError {
    fn description(&self) -> &str {
        use SerError::*;

        match *self {
            MustHaveLength => "must have lenght",
            Custom(ref s) => s,
            PackError(ref e) => e.description(),
            // Error::InvalidValueWrite(..) => "invalid value write",
            // Error::UnknownLength => {
            //     "attempt to serialize struct, sequence or map with unknown length"
            // }
            // Error::DepthLimitExceeded => "depth limit exceeded",
            // Error::Syntax(..) => "syntax error",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        use SerError::*;

        match *self {
            MustHaveLength => None,
            PackError(ref s) => Some(s),
            Custom(_) => None,
            // Error::InvalidValueWrite(ref err) => Some(err),
            // Error::UnknownLength => None,
            // Error::DepthLimitExceeded => None,
            // Error::Syntax(..) => None,
        }
    }
}

impl Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        error::Error::description(self).fmt(f)
    }
}

impl serde::ser::Error for SerError {
    fn custom<T: Display>(msg: T) -> SerError {
        SerError::Custom(msg.to_string())
    }
}
