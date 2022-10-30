use crate::utils;
use regex::Regex;
use serde::{Deserialize, Deserializer};

/// Global unique identifier for a podcast.
#[derive(Debug, PartialEq, Eq)]
pub enum Guid {
    Ok(String),
    Other(String),
}

impl std::str::FromStr for Guid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = match Regex::new(
            r"^[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}$",
        ) {
            Ok(re) => re,
            Err(e) => return Err(e.to_string()),
        };

        if !re.is_match(s) {
            return Ok(Self::Other(s.to_string()));
        }

        Ok(Self::Ok(s.to_string()))
    }
}

impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl<'de> Deserialize<'de> for Guid {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        utils::deserialize_using_from_str(d)
    }
}
