use serde::de::Error;
use serde::{Deserialize, Deserializer};

pub fn deserialize_using_from_str<'de, D, T: std::str::FromStr>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let s = match String::deserialize(d) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match T::from_str(s.as_str()) {
        Ok(t) => Ok(t),
        Err(e) => Err(e).map_err(D::Error::custom),
    }
}

pub fn from_str_exact<T: strum::IntoEnumIterator + std::fmt::Display>(s: &str) -> Option<T>
where
{
    T::iter().find(|variant| format!("{variant}") == s)
}

pub fn from_str_case_insensitive<T: strum::IntoEnumIterator + std::fmt::Display>(
    s: &str,
) -> Option<T>
where
{
    T::iter().find(|variant| format!("{variant}").to_lowercase() == s.to_lowercase())
}
