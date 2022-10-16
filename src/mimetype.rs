//// use serde_enum_str::Deserialize_enum_str;
////
//// #[derive(Debug, Deserialize_enum_str, PartialEq)]
//// pub enum Enclosure {
////     #[serde(rename = "audio/x-m4a")]
////     M4A,
////     #[serde(rename = "audio/mpeg")]
////     MP3,
////     #[serde(rename = "video/quicktime")]
////     MOV,
////     #[serde(rename = "video/mp4")]
////     MP4,
////     #[serde(rename = "video/x-m4v")]
////     M4V,
////     #[serde(rename = "application/pdf")]
////     PDF,
////
////     #[serde(other)]
////     Other(String),
//// }
////
//// #[derive(Debug, Deserialize_enum_str, PartialEq)]
//// pub enum Transcript {
////     #[serde(rename = "text/plain")]
////     Plain,
////     #[serde(rename = "text/html")]
////     HTML,
////     #[serde(rename = "text/vtt")]
////     VTT,
////     #[serde(rename = "application/json")]
////     JSON,
////     #[serde(rename = "application/x-subrip")]
////     SRT,
////     // Legacy: not supported by IANA, but was once part of podcast namespace specification.
////     #[serde(rename = "application/srt")]
////     ApplicationSRT,
////
////     #[serde(other)]
////     Other(String),
//// }
