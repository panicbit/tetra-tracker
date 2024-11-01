use std::fmt;
use std::str::FromStr;

use eyre::{Context as _, Result};
use serde::de::DeserializeOwned;
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::BOM;

pub fn deserialize_hjson<T: DeserializeOwned>(hjson: impl AsRef<[u8]>) -> Result<T> {
    let hjson = hjson.as_ref();
    let hjson = hjson.strip_prefix(BOM.as_bytes()).unwrap_or(hjson);
    let mut deserializer = serde_hjson::Deserializer::new(hjson.iter().copied());

    let items = serde_path_to_error::deserialize::<_, T>(&mut deserializer)
        .context("failed to deserialize")?;

    deserializer.end().context("trailing data")?;

    Ok(items)
}

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

pub(crate) fn option_value_or_string<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
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

    let value_or_string = Option::<ValueOrString<T>>::deserialize(deserializer)?;

    match value_or_string {
        None => Ok(None),
        Some(ValueOrString::Value(value)) => Ok(Some(value)),
        Some(ValueOrString::String(value)) => {
            value.parse::<T>().map_err(de::Error::custom).map(Some)
        }
    }
}

pub const fn const_bool<const C: bool>() -> bool {
    C
}

pub const fn const_i32<const C: i32>() -> i32 {
    C
}
