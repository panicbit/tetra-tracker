use std::fmt;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize};

pub(crate) fn value_or_string<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    T::Err: fmt::Display,
{
    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    enum ValueOrString<T> {
        Value(T),
        String(String),
    }

    let value_or_string = ValueOrString::deserialize(deserializer)?;

    match value_or_string {
        ValueOrString::Value(value) => Ok(value),
        ValueOrString::String(value) => value.parse::<T>().map_err(de::Error::custom),
    }
}
