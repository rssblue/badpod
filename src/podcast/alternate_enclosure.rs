use serde::{Deserialize, Deserializer};
use std::fmt;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{EnumIter, EnumProperty};

/// Type of [Integrity](crate::podcast::Integrity).
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum IntegrityType {
    #[strum(props(str = "sri"))]
    Sri,
    #[strum(props(str = "pgp-signature"))]
    Pgp,

    Other(String),
}

impl fmt::Display for IntegrityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => match self.get_str("str") {
                Some(s) => write!(f, "{}", s),
                None => write!(f, "{:?}", self),
            },
        }
    }
}

impl<'de> Deserialize<'de> for IntegrityType {
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
