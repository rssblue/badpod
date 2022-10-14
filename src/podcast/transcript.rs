use serde_enum_str::Deserialize_enum_str;

#[derive(Debug, Deserialize_enum_str, PartialEq)]
pub enum TranscriptRel {
    #[serde(rename = "captions")]
    Captions,

    #[serde(other)]
    Other(String),
}
