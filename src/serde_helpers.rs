use serde::{Deserializer, Serializer};
use tendril::StrTendril;

pub fn serialize_str_tendril<S>(value: &StrTendril, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(value.as_ref())
}

pub fn deserialize_str_tendril<'de, D>(deserializer: D) -> Result<StrTendril, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    Ok(StrTendril::from(s))
}
