mod de;
mod ser;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[inline]
pub fn pack<T: ?Sized>(value: &T) -> Result<Vec<u8>, ser::SerError>
where
    T: Serialize,
{
    let mut writer = vec![];
    let _ = value.serialize(&mut ser::Serializer::new(&mut writer))?;
    Ok(writer)
}

#[inline]
pub fn unpack<T>(value: &[u8]) -> Result<T, de::DeError>
where
    T: DeserializeOwned,
{
    let mut de = de::Deserializer::new(value);
    Deserialize::deserialize(&mut de)
}
