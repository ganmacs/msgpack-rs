use std::{fmt, ops, str, string};

// string type may contain invalid utf8 bytes
#[derive(Clone, Debug, PartialEq)]
pub struct Utf8String {
    pub s: Result<String, (Vec<u8>, str::Utf8Error)>,
}

impl Utf8String {
    pub fn new(s: String) -> Self {
        Self { s: Ok(s) }
    }

    pub fn is_str(&self) -> bool {
        self.s.is_ok()
    }

    pub fn is_err(&self) -> bool {
        self.s.is_err()
    }

    pub fn as_str(&self) -> Option<&str> {
        match self.s {
            Ok(ref s) => Some(s.as_str()),
            Err(_) => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        self.s.ok()
    }

    pub fn as_slice(&self) -> &[u8] {
        match &self.s {
            Ok(s) => s.as_bytes(),
            Err((v, _)) => v.as_ref(),
        }
    }

    pub fn into_err(self) -> Option<str::Utf8Error> {
        match self.s {
            Ok(_) => None,
            Err((_, err)) => Some(err),
        }
    }

    pub fn as_ref(&self) -> Utf8StringRef {
        match self.s {
            Ok(ref s) => Utf8StringRef { s: Ok(s.as_str()) },
            Err((ref buf, err)) => Utf8StringRef {
                s: Err((&buf[..], err)),
            },
        }
    }
}

impl ops::Deref for Utf8String {
    type Target = Result<String, (Vec<u8>, str::Utf8Error)>;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl fmt::Display for Utf8String {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.s {
            Ok(ref s) => write!(fmt, "\"{}\"", s),
            Err(ref err) => {
                for v in err.0.iter() {
                    write!(fmt, "\"{:X}\"", v)?
                }
                Ok(())
            }
        }
    }
}

impl From<Vec<u8>> for Utf8String {
    fn from(val: Vec<u8>) -> Self {
        match string::String::from_utf8(val) {
            Ok(s) => Utf8String::from(s),
            Err(err) => {
                let e = err.utf8_error();
                Utf8String {
                    s: Err((err.into_bytes(), e)),
                }
            }
        }
    }
}

impl From<String> for Utf8String {
    fn from(val: String) -> Self {
        Utf8String { s: Ok(val) }
    }
}

impl<'a> From<&'a str> for Utf8String {
    fn from(val: &str) -> Self {
        Utf8String { s: Ok(val.into()) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Utf8StringRef<'a> {
    pub s: Result<&'a str, (&'a [u8], str::Utf8Error)>,
}

impl<'a> Utf8StringRef<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { s: Ok(s) }
    }

    pub fn is_str(&self) -> bool {
        self.s.is_ok()
    }

    pub fn is_err(&self) -> bool {
        self.s.is_err()
    }

    pub fn as_str(&self) -> Option<&str> {
        match self.s {
            Ok(ref s) => Some(s),
            Err(_) => None,
        }
    }

    pub fn into_str(self) -> Option<String> {
        self.s.ok().map(|s| s.into())
    }

    pub fn as_slice(&self) -> &[u8] {
        match self.s {
            Ok(s) => s.as_bytes(),
            Err((v, _)) => v,
        }
    }

    pub fn into_err(self) -> Option<str::Utf8Error> {
        match self.s {
            Ok(_) => None,
            Err((_, err)) => Some(err),
        }
    }
}

impl<'a> ops::Deref for Utf8StringRef<'a> {
    type Target = Result<&'a str, (&'a [u8], str::Utf8Error)>;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> fmt::Display for Utf8StringRef<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.s {
            Ok(ref s) => write!(fmt, "\"{}\"", s),
            Err(ref err) => {
                for v in err.0.iter() {
                    write!(fmt, "\"{:X}\"", v)?
                }
                Ok(())
            }
        }
    }
}

impl<'a> From<&'a [u8]> for Utf8StringRef<'a> {
    fn from(val: &'a [u8]) -> Self {
        match str::from_utf8(val) {
            Ok(s) => Utf8StringRef::from(s),
            Err(err) => Utf8StringRef { s: Err((val, err)) },
        }
    }
}

impl<'a> From<&'a str> for Utf8StringRef<'a> {
    fn from(val: &'a str) -> Self {
        Utf8StringRef { s: Ok(val) }
    }
}

impl<'a> Into<Utf8String> for Utf8StringRef<'a> {
    fn into(self) -> Utf8String {
        match self.s {
            Ok(s) => Utf8String { s: Ok(s.into()) },
            Err((buf, err)) => Utf8String {
                s: Err((buf.into(), err)),
            },
        }
    }
}
