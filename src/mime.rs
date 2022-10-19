use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum Enclosure {
    #[strum(serialize = "audio/x-m4a")]
    AudioM4a,
    #[strum(serialize = "audio/mpeg")]
    AudioMp3,
    #[strum(serialize = "video/quicktime")]
    VideoQuicktime,
    #[strum(serialize = "video/mp4")]
    VideoMp4,
    #[strum(serialize = "video/x-m4v")]
    VideoM4v,
    #[strum(serialize = "application/pdf")]
    ApplicationPdf,
    #[strum(serialize = "audio/opus")]
    AudioOpus,
    #[strum(serialize = "audio/aac")]
    AudioAac,

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
    TextPlain,
    #[strum(serialize = "text/html")]
    TextHtml,
    #[strum(serialize = "text/vtt")]
    TextVtt,
    #[strum(serialize = "application/json")]
    ApplicationJson,
    #[strum(serialize = "application/x-subrip")]
    ApplicationSubrip,
    /// Not supported by IANA, but was once part of podcast namespace specification.
    #[deprecated]
    #[strum(serialize = "application/srt")]
    ApplicationSrt,

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

#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum Chapters {
    #[strum(serialize = "application/json+chapters")]
    ApplicationJsonChapters,
    #[strum(serialize = "application/json")]
    ApplicationJson,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for Chapters {
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
