pub mod float;
pub mod from;
pub mod integer;
pub mod utf8_string;

pub use float::Float;
pub use float::Number as FloatNumber;
pub use integer::Integer;
pub use integer::Number as IntegerNumber;
pub use utf8_string::{Utf8String, Utf8StringRef};

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    // represents an integer
    Integer(integer::Integer),

    // represents nil
    Nil,

    // represents true or false
    Boolean(bool),

    // represents a IEEE 754 double precision floating point number including NaN and Infinity
    Float(float::Float),

    // Raw. extending Raw type represents a byte array
    Binary(Vec<u8>),

    // Raw. extending Raw type represents a UTF-8 string
    String(utf8_string::Utf8String),

    // represents a sequence of objects
    Array(Vec<Value>),

    // represents key-value pairs of objects
    Map(Vec<(Value, Value)>),

    // represents a tuple of type information and a byte array where type information is an integer whose meaning is defined by applications or MessagePack specification
    Extension(i8, Vec<u8>),
    // represents an instantaneous point on the time-line in the world that is independent from time zones or calendars. Maximum precision is nanoseconds.
    // Timestamp(i64, u32),
}

#[derive(Debug, PartialEq)]
pub enum RefValue<'a> {
    // represents an integer
    Integer(integer::Integer),

    // represents nil
    Nil,

    // represents true or false
    Boolean(bool),

    // represents a IEEE 754 double precision floating point number including NaN and Infinity
    Float(float::Float),

    // Raw. extending Raw type represents a byte array
    Binary(&'a [u8]),

    // Raw. extending Raw type represents a UTF-8 string
    String(utf8_string::Utf8StringRef<'a>),

    // represents a sequence of objects
    Array(Vec<RefValue<'a>>),

    // represents key-value pairs of objects
    Map(Vec<(RefValue<'a>, RefValue<'a>)>),

    // represents a tuple of type information and a byte array where type information is an integer whose meaning is defined by applications or MessagePack specification
    Extension(i8, &'a [u8]),
    // represents an instantaneous point on the time-line in the world that is independent from time zones or calendars. Maximum precision is nanoseconds.
    // Timestamp(i64, u32),
}

impl Value {
    pub fn to_ref(&self) -> RefValue {
        match self {
            &Value::Nil => RefValue::Nil,
            &Value::Boolean(val) => RefValue::Boolean(val),
            &Value::Float(v) => RefValue::Float(v),
            &Value::Integer(val) => RefValue::Integer(val),
            &Value::Binary(ref v) => RefValue::Binary(v.as_slice()),
            &Value::String(ref v) => RefValue::String(v.as_ref()), // XXX
            &Value::Array(ref v) => RefValue::Array(v.iter().map(|v| v.to_ref()).collect()),
            &Value::Map(ref v) => RefValue::Map(
                v.iter()
                    .map(|&(ref k, ref v)| (k.to_ref(), v.to_ref()))
                    .collect(),
            ),
            &Value::Extension(ty, ref buf) => RefValue::Extension(ty, buf.as_slice()),
        }
    }
}

impl<'a> RefValue<'a> {
    pub fn to_owned(&self) -> Value {
        match self {
            &RefValue::Nil => Value::Nil,
            &RefValue::Boolean(v) => Value::Boolean(v),
            &RefValue::Integer(v) => Value::Integer(v),
            &RefValue::Float(v) => Value::Float(v),
            &RefValue::Binary(v) => Value::Binary(v.into()),
            &RefValue::String(v) => Value::String(v.into()), // XXX
            &RefValue::Array(ref v) => Value::Array(v.iter().map(|v| v.to_owned()).collect()),
            &RefValue::Map(ref v) => Value::Map(
                v.iter()
                    .map(|&(ref k, ref v)| (k.to_owned(), v.to_owned()))
                    .collect(),
            ),
            &RefValue::Extension(ty, buf) => Value::Extension(ty, buf.into()),
        }
    }
}
