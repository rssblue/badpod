use crate::utils;
use crate::Other;
use strum_macros::EnumIter;

/// Used for deserializing `rel` attribute of [Transcript](crate::podcast::Transcript).
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Rel {
    Captions,

    Other(Other),
}

impl std::str::FromStr for Rel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other((
                s.to_string(),
                "should be \"captions\"".to_string(),
            ))),
        }
    }
}

impl std::fmt::Display for Rel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other((s, _)) => write!(f, "{s}"),
            _ => {
                let s = format!("{:?}", self);
                write!(f, "{}", s.to_lowercase())
            }
        }
    }
}

impl Rel {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(variant) => variant,
            Err(e) => Self::Other((s.to_string(), e)),
        }
    }
}
