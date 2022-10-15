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

#[derive(Debug, PartialEq)]
pub enum U64 {
    U64(u64),
    Other(String),
}

pub fn option_u64<'de, D>(deserializer: D) -> Result<Option<U64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.parse::<u64>() {
        Ok(number) => Ok(Some(U64::U64(number))),
        _ => Ok(Some(U64::Other(s))),
    }
}
