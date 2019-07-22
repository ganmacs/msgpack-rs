use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Number {
    Float32(f32),
    Float64(f64),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Float {
    pub n: Number,
}

impl From<f64> for Float {
    #[inline]
    fn from(u: f64) -> Self {
        Float {
            n: { Number::Float64(u) },
        }
    }
}

impl From<f32> for Float {
    #[inline]
    fn from(u: f32) -> Self {
        Float {
            n: { Number::Float32(u) },
        }
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.n {
            Number::Float32(v) => write!(f, "{}", v),
            Number::Float64(v) => write!(f, "{}", v),
        }
    }
}
