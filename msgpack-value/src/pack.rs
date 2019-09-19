use crate::{value, Value};

use msgpack::{pack, PackError};
use std::io;

pub fn pack_value<W: io::Write>(writer: &mut W, val: Value) -> Result<(), PackError> {
    match val {
        Value::Integer(value::integer::Integer { n }) => match n {
            value::integer::Number::PosInt(v) => pack::pack_from_u64(writer, v),
            value::integer::Number::NegInt(v) => pack::pack_from_i64(writer, v),
        },
        Value::Nil => pack::pack_nil(writer),
        Value::Boolean(v) => pack::pack_bool(writer, v),
        Value::Float(value::float::Float { n }) => match n {
            value::float::Number::Float32(v) => pack::pack_f32(writer, v),
            value::float::Number::Float64(v) => pack::pack_f64(writer, v),
        },
        Value::Binary(v) => pack::pack_bin(writer, &v),
        Value::String(value::utf8_string::Utf8String { ref s }) => match *s {
            Ok(ref s) => pack::pack_str(writer, &s.as_str()),
            Err((ref s, _)) => pack::pack_str_from_slice(writer, s),
        },
        Value::Array(vs) => {
            pack::pack_array_header(writer, vs.len())?;
            for v in vs {
                pack_value(writer, v)?;
            }
            Ok(())
        }
        Value::Map(vs) => {
            pack::pack_map_header(writer, vs.len())?;
            for (k, v) in vs {
                pack_value(writer, k)?;
                pack_value(writer, v)?;
            }
            Ok(())
        }
        Value::Extension(tag, v) => {
            pack::pack_ext_header(writer, tag, v.len())?;
            pack::write_all(writer, &v)
        }
        Value::Timestamp(sec, nsec) => pack::pack_timestamp(writer, sec, nsec),
    }
}
