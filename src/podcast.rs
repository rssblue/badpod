use serde::Deserialize;

use crate::bool;
use crate::language::Language;

mod transcript;
pub use transcript::TranscriptRel;

mod chapters;
pub use chapters::ChaptersType;

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
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

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Locked {
    #[serde(rename = "$attr:owner")]
    pub owner: Option<String>,
    #[serde(rename = "$value", deserialize_with = "bool::option_bool_yn")]
    pub value: Option<bool::Bool>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Funding {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Chapters {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$attr:type")]
    pub type_: Option<ChaptersType>,
}
