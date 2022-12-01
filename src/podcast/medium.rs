use crate::utils;
use strum_macros::EnumIter;

/// Medium of the feed.
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Medium {
    Podcast,
    Music,
    Video,
    Film,
    Audiobook,
    Newsletter,
    Blog,

    Other(String),
}

impl std::str::FromStr for Medium {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for Medium {
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

impl Medium {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(medium) => medium,
            Err(_) => Self::Other(s.to_string()),
        }
    }
}
