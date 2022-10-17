use regex::Regex;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq)]
pub enum GUID {
    GUID(String),
    Other(String),
}

impl<'de> Deserialize<'de> for GUID {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let re = match Regex::new(
            r"^[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}$",
        ) {
            Ok(re) => re,
            Err(e) => return Err(e.to_string()).map_err(D::Error::custom),
        };

        if !re.is_match(s.as_str()) {
            return Ok(GUID::Other(s));
        }

        Ok(GUID::GUID(s))
    }
}
