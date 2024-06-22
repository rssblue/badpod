use crate::utils;
use crate::Other;
use strum::EnumIter;

/// Type of [Value](crate::podcast::Value).
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Type {
    Bitcoin,
    Lightning,
    Amp,

    Other(Other),
}

impl std::str::FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other((
                s.to_string(),
                "should be one of the following: \"bitcoin\", \"lightning\", \"amp\"".to_string(),
            ))),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other((s, _)) => write!(f, "{s}"),
            _ => {
                let s = format!("{self:?}");
                write!(f, "{}", s.to_lowercase())
            }
        }
    }
}

impl Type {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(variant) => variant,
            Err(e) => Self::Other((s.to_string(), e)),
        }
    }
}

/// Method of [Value](crate::podcast::Value).
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Method {
    Default,
    Keysend,

    Other(Other),
}

impl std::str::FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other((
                s.to_string(),
                "unrecognized method".to_string(),
            ))),
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other((s, _)) => write!(f, "{s}"),
            _ => {
                let s = format!("{self:?}");
                write!(f, "{}", s.to_lowercase())
            }
        }
    }
}

impl Method {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(variant) => variant,
            Err(e) => Self::Other((s.to_string(), e)),
        }
    }
}

/// Type of [ValueRecipient](crate::podcast::ValueRecipient).
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum RecipientType {
    Wallet,
    Node,

    Other(Other),
}

impl std::str::FromStr for RecipientType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other((
                s.to_string(),
                "should be either \"wallet\" or \"node\"".to_string(),
            ))),
        }
    }
}

impl std::fmt::Display for RecipientType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other((s, _)) => write!(f, "{s}"),
            _ => {
                let s = format!("{self:?}");
                write!(f, "{}", s.to_lowercase())
            }
        }
    }
}

impl RecipientType {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(variant) => variant,
            Err(e) => Self::Other((s.to_string(), e)),
        }
    }
}
