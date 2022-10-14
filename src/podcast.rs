use serde::Deserialize;

use crate::bool;
use crate::language::Language;

mod transcript;
pub use transcript::TranscriptRel;

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
    #[serde(rename = "$value")]
    #[serde(deserialize_with = "bool::option_bool_yn")]
    pub value: Option<bool::Bool>,
}
