use serde::{Deserialize, Deserializer};
use std::fmt;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{EnumIter, EnumProperty};

/// Social protocols that can be used in [SocialInteract](crate::podcast::SocialInteract).
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum SocialProtocol {
    #[strum(props(str = "disabled"))]
    Disabled,
    #[strum(props(str = "activitypub"))]
    ActivityPub,
    #[strum(props(str = "twitter"))]
    Twitter,
    // TODO: change to lowercase.
    #[strum(props(str = "Lightning"))]
    Lightning,

    Other(String),
}

impl fmt::Display for SocialProtocol {
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

impl<'de> Deserialize<'de> for SocialProtocol {
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
