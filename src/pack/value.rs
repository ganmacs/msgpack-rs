use super::primitive::write_all;
use super::{
    pack_array_header, pack_bin, pack_bool, pack_ext_header, pack_f32, pack_f64, pack_from_i64,
    pack_from_u64, pack_map_header, pack_nil, pack_str, pack_str_from_slice, pack_timestamp,
};
use crate::pack_error::PackError;

use crate::value;
use crate::value::float::{self, Float};
use crate::value::integer::{self, Integer};
use crate::value::Value;
use std::io;

pub fn pack_value<W: io::Write>(writer: &mut W, val: Value) -> Result<(), PackError> {
    match val {
        Value::Integer(Integer { n }) => match n {
            integer::Number::PosInt(v) => pack_from_u64(writer, v),
            integer::Number::NegInt(v) => pack_from_i64(writer, v),
        },
        Value::Nil => pack_nil(writer),
        Value::Boolean(v) => pack_bool(writer, v),
        Value::Float(Float { n }) => match n {
            float::Number::Float32(v) => pack_f32(writer, v),
            float::Number::Float64(v) => pack_f64(writer, v),
        },
        Value::Binary(v) => pack_bin(writer, &v),
        Value::String(value::Utf8String { ref s }) => {
            match *s {
                Ok(ref s) => pack_str(writer, &s.as_str()),
                Err((ref s, _)) => pack_str_from_slice(writer, s),
            }
        }
        Value::Array(vs) => {
            pack_array_header(writer, vs.len())?;
            for v in vs {
                pack_value(writer, v)?;
            }
            Ok(())
        }
        Value::Map(vs) => {
            pack_map_header(writer, vs.len())?;
            for (k, v) in vs {
                pack_value(writer, k)?;
                pack_value(writer, v)?;
            }
            Ok(())
        }
        Value::Extension(tag, v) => {
            pack_ext_header(writer, tag, v.len())?;
            write_all(writer, &v)
        }
        Value::Timestamp(sec, nsec) => pack_timestamp(writer, sec, nsec),
    }
}
