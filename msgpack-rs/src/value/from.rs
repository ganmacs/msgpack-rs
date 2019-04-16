use crate::value::Value;

macro_rules! from_pos_integer {
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

from_pos_integer! {u8 u16 u32 u64 usize}

macro_rules! from_neg_integer {
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

from_neg_integer! {i8 i16 i32 i64 isize}

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
