use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Used for deserializing `rel` attribute of [Transcript](crate::podcast::Transcript).
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum TranscriptRel {
    #[strum(serialize = "captions")]
    Captions,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for TranscriptRel {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match TranscriptRel::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(TranscriptRel::Other(s)),
        }
    }
}
