use crate::utils;
use strum::EnumProperty;
use strum_macros::{EnumIter, EnumProperty};

/// Type of [Integrity](crate::podcast::Integrity).
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum IntegrityType {
    #[strum(props(str = "sri"))]
    Sri,
    #[strum(props(str = "pgp-signature"))]
    Pgp,

    Other(String),
}

impl std::str::FromStr for IntegrityType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for IntegrityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => match self.get_str("str") {
                Some(s) => write!(f, "{}", s),
                None => write!(f, "{:?}", self),
            },
        }
    }
}

impl IntegrityType {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(variant) => variant,
            Err(_) => Self::Other(s.to_string()),
        }
    }
}
