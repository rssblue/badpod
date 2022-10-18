use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum ChaptersType {
    #[strum(serialize = "application/json+chapters")]
    ApplicationJsonChapters,
    #[strum(serialize = "application/json")]
    ApplicationJson,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for ChaptersType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match ChaptersType::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(ChaptersType::Other(s)),
        }
    }
}
