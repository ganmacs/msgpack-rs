pub mod float;
pub mod from;
pub mod integer;

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
    String(String),

    // represents a sequence of objects
    Array(Vec<Value>),

    // represents key-value pairs of objects
    Map(Vec<(Value, Value)>),

    // represents a tuple of type information and a byte array where type information is an integer whose meaning is defined by applications or MessagePack specification
    Extension(i8, Vec<u8>),
    // represents an instantaneous point on the time-line in the world that is independent from time zones or calendars. Maximum precision is nanoseconds.
    // Timestamp(i64, u32),
}

pub enum ValueRef<'a> {
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
    String(&'a str),

    // represents a sequence of objects
    Array(Vec<ValueRef<'a>>),

    // represents key-value pairs of objects
    Map(Vec<(ValueRef<'a>, ValueRef<'a>)>),

    // represents a tuple of type information and a byte array where type information is an integer whose meaning is defined by applications or MessagePack specification
    Extension(i8, &'a [u8]),
    // represents an instantaneous point on the time-line in the world that is independent from time zones or calendars. Maximum precision is nanoseconds.
    // Timestamp(i64, u32),
}

impl Value {
    pub fn to_ref(&self) -> ValueRef {
        match self {
            &Value::Nil => ValueRef::Nil,
            &Value::Boolean(val) => ValueRef::Boolean(val),
            &Value::Float(v) => ValueRef::Float(v),
            &Value::Integer(val) => ValueRef::Integer(val),
            &Value::Binary(ref v) => ValueRef::Binary(v.as_slice()),
            &Value::String(ref v) => ValueRef::String(v), // XXX
            &Value::Array(ref v) => ValueRef::Array(v.iter().map(|v| v.to_ref()).collect()),
            &Value::Map(ref v) => ValueRef::Map(
                v.iter()
                    .map(|&(ref k, ref v)| (k.to_ref(), v.to_ref()))
                    .collect(),
            ),
            &Value::Extension(ty, ref buf) => ValueRef::Extension(ty, buf.as_slice()),
        }
    }
}

impl<'a> ValueRef<'a> {
    pub fn to_owned(&self) -> Value {
        match self {
            &ValueRef::Nil => Value::Nil,
            &ValueRef::Boolean(v) => Value::Boolean(v),
            &ValueRef::Integer(v) => Value::Integer(v),
            &ValueRef::Float(v) => Value::Float(v),
            &ValueRef::Binary(v) => Value::Binary(v.into()),
            &ValueRef::String(v) => Value::String(v.into()), // XXX
            &ValueRef::Array(ref v) => Value::Array(v.iter().map(|v| v.to_owned()).collect()),
            &ValueRef::Map(ref v) => Value::Map(
                v.iter()
                    .map(|&(ref k, ref v)| (k.to_owned(), v.to_owned()))
                    .collect(),
            ),
            &ValueRef::Extension(ty, buf) => Value::Extension(ty, buf.into()),
        }
    }
}
