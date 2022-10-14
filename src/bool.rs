use serde::{Deserialize, Deserializer};

pub fn option_bool_tf<'de, D>(deserializer: D) -> Result<Option<BoolTF>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.as_str() {
        "false" => Ok(Some(BoolTF::Bool(false))),
        "true" => Ok(Some(BoolTF::Bool(false))),
        _ => Ok(Some(BoolTF::Other(s))),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoolTF {
    Bool(bool),
    Other(String),
}
