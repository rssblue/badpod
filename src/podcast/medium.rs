use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum Medium {
    #[strum(serialize = "podcast")]
    Podcast,
    #[strum(serialize = "music")]
    Music,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "film")]
    Film,
    #[strum(serialize = "audiobook")]
    Audiobook,
    #[strum(serialize = "newsletter")]
    Newsletter,
    #[strum(serialize = "blog")]
    Blog,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for Medium {
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
