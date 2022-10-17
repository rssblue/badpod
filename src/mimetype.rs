use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum Enclosure {
    #[strum(serialize = "audio/x-m4a")]
    M4A,
    #[strum(serialize = "audio/mpeg")]
    MP3,
    #[strum(serialize = "video/quicktime")]
    MOV,
    #[strum(serialize = "video/mp4")]
    MP4,
    #[strum(serialize = "video/x-m4v")]
    M4V,
    #[strum(serialize = "application/pdf")]
    PDF,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for Enclosure {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match Enclosure::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(Enclosure::Other(s)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum Transcript {
    #[strum(serialize = "text/plain")]
    Plain,
    #[strum(serialize = "text/html")]
    HTML,
    #[strum(serialize = "text/vtt")]
    VTT,
    #[strum(serialize = "application/json")]
    JSON,
    #[strum(serialize = "application/x-subrip")]
    SRT,
    /// Not supported by IANA, but was once part of podcast namespace specification.
    #[deprecated]
    #[strum(serialize = "application/srt")]
    ApplicationSRT,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for Transcript {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match Transcript::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(Transcript::Other(s)),
        }
    }
}
