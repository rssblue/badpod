use serde_enum_str::Deserialize_enum_str;

#[derive(Debug, Deserialize_enum_str, PartialEq, Eq)]
pub enum ChaptersType {
    #[serde(rename = "application/json+chapters")]
    ApplicationJSONChapters,
    #[serde(rename = "application/json")]
    ApplicationJSON,

    #[serde(other)]
    Other(String),
}
