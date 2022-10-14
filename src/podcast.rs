use serde::Deserialize;

use crate::bool;
use crate::language::Language;
use crate::numbers;

mod transcript;
pub use transcript::TranscriptRel;

mod chapters;
pub use chapters::ChaptersType;

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Transcript {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$attr:type")]
    pub type_: Option<String>,
    #[serde(rename = "$attr:language")]
    pub language: Option<Language>,
    #[serde(rename = "$attr:rel")]
    pub rel: Option<TranscriptRel>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Locked {
    #[serde(rename = "$attr:owner")]
    pub owner: Option<String>,
    #[serde(rename = "$value", deserialize_with = "bool::option_bool_yn")]
    pub value: Option<bool::Bool>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Funding {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Chapters {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$attr:type")]
    pub type_: Option<ChaptersType>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Soundbite {
    #[serde(rename = "$attr:startTime", deserialize_with = "numbers::option_float")]
    pub start_time: Option<numbers::Float>,
    #[serde(rename = "$attr:duration", deserialize_with = "numbers::option_float")]
    pub duration: Option<numbers::Float>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}
