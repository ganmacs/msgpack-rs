use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Number {
    PosInt(u64),
    NegInt(i64),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Integer {
    pub n: Number,
}

macro_rules! impl_from_pos_integer {
    ($($ty:ty)*) => {
        $(
            impl From<$ty> for Integer {
                #[inline]
                fn from(u: $ty) -> Self {
                    Integer { n: { Number::PosInt(u as u64) } }
                }
            }
        )*
    };
}

impl_from_pos_integer! {u8 u16 u32 u64 usize}

macro_rules! impl_from_neg_integer {
    ($($ty:ty)*) => {
        $(
            impl From<$ty> for Integer {
                #[inline]
                fn from(u: $ty) -> Self {
                    Integer { n: { Number::NegInt(u as i64) } }
                }
            }
        )*
    };
}

impl_from_neg_integer! {i8 i16 i32 i64 isize}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.n {
            Number::PosInt(v) => write!(f, "{}", v),
            Number::NegInt(v) => write!(f, "{}", v),
        }
    }
}
