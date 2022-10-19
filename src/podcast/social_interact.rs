use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Social protocols that can be used in [SocialInteract](crate::podcast::SocialInteract).
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum SocialProtocol {
    #[strum(serialize = "disabled")]
    Disabled,
    #[strum(serialize = "activitypub")]
    ActivityPub,
    #[strum(serialize = "twitter")]
    Twitter,
    #[strum(serialize = "Lightning")]
    Lightning,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for SocialProtocol {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match Self::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}
