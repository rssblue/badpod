use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum DateTime {
    Ok(chrono::DateTime<chrono::FixedOffset>),
    Other(String),
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match chrono::DateTime::parse_from_rfc2822(&s) {
            Ok(t) => Ok(Self::Ok(t)),
            Err(_) => Ok(DateTime::Other(s.to_string())),
        }
    }
}

pub fn option_datetime_iso8601<'de, D>(d: D) -> Result<Option<DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(d) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match chrono::DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f%:z") {
        Ok(t) => Ok(Some(DateTime::Ok(t))),
        Err(_) => Ok(Some(DateTime::Other(s.to_string()))),
    }
}
