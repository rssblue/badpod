use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Type of [Value](crate::podcast::Value).
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum ValueType {
    #[strum(serialize = "bitcoin")]
    Bitcoin,
    #[strum(serialize = "lightning")]
    Lightning,
    #[strum(serialize = "amp")]
    Amp,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for ValueType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match Self::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}

/// Method of [Value](crate::podcast::Value).
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum ValueMethod {
    #[strum(serialize = "default")]
    Default,
    #[strum(serialize = "keysend")]
    Keysend,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for ValueMethod {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match Self::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}

/// Type of [ValueRecipient](crate::podcast::ValueRecipient).
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum ValueRecipientType {
    #[strum(serialize = "wallet")]
    Wallet,
    #[strum(serialize = "node")]
    Node,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for ValueRecipientType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match Self::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}
