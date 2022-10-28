use serde::{Deserialize, Deserializer};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Medium of the feed.
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Medium {
    Podcast,
    Music,
    Video,
    Film,
    Audiobook,
    Newsletter,
    Blog,

    Other(String),
}

impl fmt::Display for Medium {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => {
                let s = format!("{:?}", self);
                write!(f, "{}", s.to_lowercase())
            }
        }
    }
}

impl<'de> Deserialize<'de> for Medium {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        for variant in Self::iter() {
            if format!("{variant}") == s {
                return Ok(variant);
            };
        }

        Ok(Self::Other(s))
    }
}
