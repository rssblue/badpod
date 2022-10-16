use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum Bool {
    Bool(bool),
    Other(String),
}

pub fn option_bool_tf<'de, D>(deserializer: D) -> Result<Option<Bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.as_str() {
        "false" => Ok(Some(Bool::Bool(false))),
        "true" => Ok(Some(Bool::Bool(true))),
        _ => Ok(Some(Bool::Other(s))),
    }
}

pub fn option_bool_yn<'de, D>(deserializer: D) -> Result<Option<Bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.as_str() {
        "no" => Ok(Some(Bool::Bool(false))),
        "yes" => Ok(Some(Bool::Bool(false))),
        _ => Ok(Some(Bool::Other(s))),
    }
}
