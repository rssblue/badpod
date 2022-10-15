use serde_enum_str::Deserialize_enum_str;

#[derive(Debug, Deserialize_enum_str, PartialEq)]
pub enum Enclosure {
    #[serde(rename = "audio/x-m4a")]
    M4A,
    #[serde(rename = "audio/mpeg")]
    MP3,
    #[serde(rename = "video/quicktime")]
    MOV,
    #[serde(rename = "video/mp4")]
    MP4,
    #[serde(rename = "video/x-m4v")]
    M4V,
    #[serde(rename = "application/pdf")]
    PDF,

    #[serde(other)]
    Other(String),
}
