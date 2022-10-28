use serde::{Deserialize, Deserializer};
use std::fmt;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{EnumIter, EnumProperty};

/// Used for deserializing mime types of enclosures.
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum Enclosure {
    #[strum(props(str = "audio/x-m4a"))]
    AudioM4a,
    #[strum(props(str = "audio/mpeg"))]
    AudioMp3,
    #[strum(props(str = "video/quicktime"))]
    VideoQuicktime,
    #[strum(props(str = "video/mp4"))]
    VideoMp4,
    #[strum(props(str = "video/x-m4v"))]
    VideoM4v,
    #[strum(props(str = "application/pdf"))]
    ApplicationPdf,
    #[strum(props(str = "audio/opus"))]
    AudioOpus,
    #[strum(props(str = "audio/aac"))]
    AudioAac,

    Other(String),
}

impl fmt::Display for Enclosure {
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

impl<'de> Deserialize<'de> for Enclosure {
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

/// Used for deserializing mime types of transcripts.
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum Transcript {
    #[strum(props(str = "text/plain"))]
    TextPlain,
    #[strum(props(str = "text/html"))]
    TextHtml,
    #[strum(props(str = "text/vtt"))]
    TextVtt,
    #[strum(props(str = "application/json"))]
    ApplicationJson,
    #[strum(props(str = "application/x-subrip"))]
    ApplicationSubrip,
    /// Not supported by IANA, but was once part of podcast namespace specification.
    #[deprecated]
    #[strum(props(str = "application/srt"))]
    ApplicationSrt,

    Other(String),
}

impl fmt::Display for Transcript {
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

impl<'de> Deserialize<'de> for Transcript {
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

/// Used for deserializing mime types of chapters.
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum Chapters {
    #[strum(props(str = "application/json+chapters"))]
    ApplicationJsonChapters,
    #[strum(props(str = "application/json"))]
    ApplicationJson,

    Other(String),
}

impl fmt::Display for Chapters {
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

impl<'de> Deserialize<'de> for Chapters {
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
