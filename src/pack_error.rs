use std::error;
use std::fmt::{self, Display, Formatter};
use std::io::Error;

#[derive(Debug)]
pub enum PackError {
    WriteError(Error),
}

impl From<Error> for PackError {
    fn from(err: Error) -> PackError {
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
