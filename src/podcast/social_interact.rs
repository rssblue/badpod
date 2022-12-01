use crate::utils;
use strum_macros::EnumIter;

/// Social protocols that can be used in [SocialInteract](crate::podcast::SocialInteract).
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Protocol {
    Disabled,
    ActivityPub,
    Twitter,
    Lightning,

    Other(String),
}

impl std::str::FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => {
                let s = format!("{:?}", self);
                write!(f, "{}", s.to_lowercase())
            }
        }
    }
}

impl Protocol {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(protocol) => protocol,
            Err(_) => Self::Other(s.to_string()),
        }
    }
}
