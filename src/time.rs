use serde::{Deserialize, Deserializer};

pub fn option_datefmt<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match chrono::DateTime::parse_from_rfc2822(&s) {
        Ok(t) => Ok(Some(DateTime::Rfc2822(t))),
        Err(_) => Ok(Some(DateTime::Other(s.to_string()))),
    }
}

#[derive(Debug, PartialEq)]
pub enum DateTime {
    Rfc2822(chrono::DateTime<chrono::FixedOffset>),
    Other(String),
}

