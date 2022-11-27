use crate::utils;
use serde::{Deserialize, Deserializer};

/// Used for deserializing combined dates and times.
#[derive(Debug, PartialEq, Eq)]
pub enum DateTime {
    Ok(chrono::DateTime<chrono::FixedOffset>),
    Other(String),
}

impl std::str::FromStr for DateTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match chrono::DateTime::parse_from_rfc2822(s) {
            Ok(t) => Ok(Self::Ok(t)),
            Err(_) => Ok(DateTime::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        utils::deserialize_using_from_str(d)
    }
}

pub enum DatetimeISO8601 {}

impl<'de> serde_with::DeserializeAs<'de, DateTime> for DatetimeISO8601 {
    fn deserialize_as<D: Deserializer<'de>>(d: D) -> Result<DateTime, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match chrono::DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f%:z") {
            Ok(t) => Ok(DateTime::Ok(t)),
            Err(_) => Ok(DateTime::Other(s.to_string())),
        }
    }
}
