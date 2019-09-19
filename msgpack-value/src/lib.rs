mod pack;
mod unpack;
mod unpacker;
mod value;

pub use pack::pack_value;
pub use unpack::{unpack_value, unpack_value_ref};
pub use unpacker::*;

// pub//  mod float;
// pub mod from;
// pub mod integer;
// pub mod utf8_string;

// pub use float::Float;
// pub use float::Number as FloatNumber;
// pub use integer::Integer;
// pub use integer::Number as IntegerNumber;
// use std::iter::Iterator;
// pub use utf8_string::{Utf8String, Utf8StringRef};

// use chrono::{self, TimeZone};
// use std::fmt;
// pub struct Nil;

use msgpack::PackError;
use std::io;

pub struct ValuePacker<W> {
    wr: W,
}

impl<W> ValuePacker<W> {
    pub fn new(wr: W) -> Self {
        ValuePacker { wr }
    }
}

impl<W: io::Write> ValuePacker<W> {
    pub fn pack_value(&mut self, v: Value) -> Result<(), PackError> {
        pack::pack_value(&mut self.wr, v)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    // represents an integer
    Integer(value::Integer),

    // represents nil
    Nil,

    // represents true or false
    Boolean(bool),

    // represents a IEEE 754 double precision floating point number including NaN and Infinity
    Float(value::Float),

    // Raw. extending Raw type represents a byte array
    Binary(Vec<u8>),

    // Raw. extending Raw type represents a UTF-8 string
    String(value::Utf8String),

    // represents a sequence of objects
    Array(Vec<Value>),

    // represents key-value pairs of objects
    Map(Vec<(Value, Value)>),

    // represents a tuple of type information and a byte array where type information is an integer whose meaning is defined by applications or MessagePack specification
    Extension(i8, Vec<u8>),

    // represents an instantaneous point on the time-line in the world that is independent from time zones or calendars. Maximum precision is nanoseconds.
    Timestamp(i64, u32),
}

// impl fmt::Display for Value {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Value::Nil => write!(f, "null"),
//             Value::Boolean(val) => write!(f, "{}", if *val { "true" } else { "false" }),
//             Value::Float(val) => val.fmt(f),
//             Value::Integer(val) => val.fmt(f),
//             Value::Binary(ref val) => {
//                 for v in val {
//                     write!(f, "{:02X}", v)?;
//                 }
//                 Ok(())
//             }
//             Value::String(ref val) => val.fmt(f),
//             Value::Array(ref val) => {
//                 write!(f, "[")?;
//                 let mut s = true;
//                 for v in val {
//                     if s {
//                         s = false
//                     } else {
//                         write!(f, ", ")?;
//                     }

//                     v.fmt(f)?;
//                 }
//                 write!(f, "]")
//             }
//             Value::Map(ref val) => {
//                 write!(f, "{{")?;
//                 let mut s = true;
//                 for v in val {
//                     if s {
//                         s = false
//                     } else {
//                         write!(f, ", ")?;
//                     }

//                     write!(f, "{}: {}", v.0, v.1)?;
//                 }
//                 write!(f, "}}")
//             }
//             Value::Extension(ty, ref buf) => {
//                 write!(f, "Extension({}, ", ty)?;
//                 for b in buf {
//                     write!(f, "{:X}", b)?;
//                 }
//                 write!(f, ")")
//             }

//             Value::Timestamp(sec, nsec) => {
//                 write!(f, "{}", chrono::Local.timestamp(*sec as i64, *nsec))
//             }
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub enum RefValue<'a> {
    // represents an integer
    Integer(value::Integer),

    // represents nil
    Nil,

    // represents true or false
    Boolean(bool),

    // represents a IEEE 754 double precision floating point number including NaN and Infinity
    Float(value::Float),

    // Raw. extending Raw type represents a byte array
    Binary(&'a [u8]),

    // Raw. extending Raw type represents a UTF-8 string
    String(value::Utf8StringRef<'a>),

    // represents a sequence of objects
    Array(Vec<RefValue<'a>>),

    // represents key-value pairs of objects
    Map(Vec<(RefValue<'a>, RefValue<'a>)>),

    // represents a tuple of type information and a byte array where type information is an integer whose meaning is defined by applications or MessagePack specification
    Extension(i8, &'a [u8]),

    // represents an instantaneous point on the time-line in the world that is independent from time zones or calendars. Maximum precision is nanoseconds.
    Timestamp(i64, u32),
}

// impl<'a> fmt::Display for RefValue<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             RefValue::Nil => write!(f, "null"),
//             RefValue::Boolean(val) => write!(f, "{}", if *val { "true" } else { "false" }),
//             RefValue::Float(val) => val.fmt(f),
//             RefValue::Integer(val) => val.fmt(f),
//             RefValue::Binary(val) => {
//                 for v in *val {
//                     write!(f, "{:02X}", v)?;
//                 }
//                 Ok(())
//             }
//             RefValue::String(ref val) => val.fmt(f),
//             RefValue::Array(ref val) => {
//                 write!(f, "[")?;
//                 let mut s = true;
//                 for v in val {
//                     if s {
//                         s = false
//                     } else {
//                         write!(f, ", ")?;
//                     }

//                     v.fmt(f)?;
//                 }
//                 write!(f, "]")
//             }
//             RefValue::Map(ref val) => {
//                 write!(f, "{{")?;
//                 let mut s = true;
//                 for v in val {
//                     if s {
//                         s = false
//                     } else {
//                         write!(f, ", ")?;
//                     }

//                     write!(f, "{}: {}", v.0, v.1)?;
//                 }
//                 write!(f, "}}")
//             }
//             RefValue::Extension(ty, buf) => {
//                 write!(f, "Extension({}, ", ty)?;
//                 for b in *buf {
//                     write!(f, "{:X}", b)?;
//                 }
//                 write!(f, ")")
//             }
//             RefValue::Timestamp(sec, nsec) => {
//                 write!(f, "{}", chrono::Local.timestamp(*sec as i64, *nsec))
//             }
//         }
//     }
// }

// impl Value {
//     pub fn to_ref(&self) -> RefValue {
//         match self {
//             &Value::Nil => RefValue::Nil,
//             &Value::Boolean(val) => RefValue::Boolean(val),
//             &Value::Float(v) => RefValue::Float(v),
//             &Value::Integer(val) => RefValue::Integer(val),
//             &Value::Binary(ref v) => RefValue::Binary(v.as_slice()),
//             &Value::String(ref v) => RefValue::String(v.as_ref()), // XXX
//             &Value::Array(ref v) => RefValue::Array(v.iter().map(|v| v.to_ref()).collect()),
//             &Value::Map(ref v) => RefValue::Map(
//                 v.iter()
//                     .map(|&(ref k, ref v)| (k.to_ref(), v.to_ref()))
//                     .collect(),
//             ),
//             &Value::Extension(ty, ref buf) => RefValue::Extension(ty, buf.as_slice()),
//             &Value::Timestamp(sec, nsec) => RefValue::Timestamp(sec, nsec),
//         }
//     }

//     pub fn to_nil(&self) -> Option<Nil> {
//         match self {
//             Value::Nil => Some(Nil),
//             _ => None,
//         }
//     }

//     pub fn to_array(&self) -> Option<&Vec<Value>> {
//         match self {
//             Value::Array(v) => Some(v),
//             _ => None,
//         }
//     }
// }

// impl<'a> RefValue<'a> {
//     pub fn to_owned(&self) -> Value {
//         match self {
//             &RefValue::Nil => Value::Nil,
//             &RefValue::Boolean(v) => Value::Boolean(v),
//             &RefValue::Integer(v) => Value::Integer(v),
//             &RefValue::Float(v) => Value::Float(v),
//             &RefValue::Binary(v) => Value::Binary(v.into()),
//             &RefValue::String(v) => Value::String(v.into()), // XXX
//             &RefValue::Array(ref v) => Value::Array(v.iter().map(|v| v.to_owned()).collect()),
//             &RefValue::Map(ref v) => Value::Map(
//                 v.iter()
//                     .map(|&(ref k, ref v)| (k.to_owned(), v.to_owned()))
//                     .collect(),
//             ),
//             &RefValue::Extension(ty, buf) => Value::Extension(ty, buf.into()),
//             &RefValue::Timestamp(sec, nsec) => Value::Timestamp(sec, nsec),
//         }
//     }

//     pub fn to_nil(&self) -> Option<Nil> {
//         match self {
//             RefValue::Nil => Some(Nil),
//             _ => None,
//         }
//     }

//     pub fn to_array(&self) -> Option<&Vec<RefValue>> {
//         match self {
//             RefValue::Array(v) => Some(v),
//             _ => None,
//         }
//     }

//     pub fn to_str(&self) -> Option<&str> {
//         match self {
//             RefValue::String(v) => v.as_str(),
//             _ => None,
//         }
//     }

//     pub fn to_slice(&self) -> Option<&[u8]> {
//         match self {
//             RefValue::String(v) => Some(v.as_slice()),
//             _ => None,
//         }
//     }
// }
