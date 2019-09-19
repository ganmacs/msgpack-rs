use crate::{RefValue, Value};

macro_rules! from_pos_value_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Value {
                fn from(n: $ty) -> Self {
                    Value::Integer(n.into())
                }
            }
        )*
    };
}

from_pos_value_integer! {u8 u16 u32 u64 usize}

macro_rules! from_neg_value_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Value {
                fn from(n: $ty) -> Self {
                    Value::Integer(n.into())
                }
            }
        )*
    };
}

from_neg_value_integer! {i8 i16 i32 i64 isize}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(v.into())
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v.into())
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Boolean(v)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::String(super::utf8_string::Utf8String::new(v))
    }
}

macro_rules! from_pos_value_ref_integer {
    ($($ty:ident)*) => {
        $(
            impl<'a> From<$ty> for RefValue<'a> {
                fn from(n: $ty) -> Self {
                    RefValue::Integer(n.into())
                }
            }
        )*
    };
}

from_pos_value_ref_integer! {u8 u16 u32 u64 usize}

macro_rules! from_neg_value_ref_integer {
    ($($ty:ident)*) => {
        $(
            impl<'a> From<$ty> for RefValue<'a> {
                fn from(n: $ty) -> Self {
                    RefValue::Integer(n.into())
                }
            }
        )*
    };
}

from_neg_value_ref_integer! {i8 i16 i32 i64 isize}

impl<'a> From<f32> for RefValue<'a> {
    fn from(v: f32) -> Self {
        RefValue::Float(v.into())
    }
}

impl<'a> From<f64> for RefValue<'a> {
    fn from(v: f64) -> Self {
        RefValue::Float(v.into())
    }
}

impl<'a> From<bool> for RefValue<'a> {
    fn from(v: bool) -> Self {
        RefValue::Boolean(v)
    }
}

impl<'a> From<&'a str> for RefValue<'a> {
    fn from(v: &'a str) -> Self {
        RefValue::String(super::utf8_string::Utf8StringRef::new(v))
    }
}
