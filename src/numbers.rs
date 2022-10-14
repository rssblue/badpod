use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum Float {
    Float(f32),
    Other(String),
}

pub fn option_float<'de, D>(deserializer: D) -> Result<Option<Float>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.parse::<f32>() {
        Ok(number) => Ok(Some(Float::Float(number))),
        _ => Ok(Some(Float::Other(s))),
    }
}
