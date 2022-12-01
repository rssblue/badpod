use crate::utils;
use strum_macros::EnumIter;

/// Allowed purposes for [Txt](crate::podcast::Txt).
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Purpose {
    Verify,

    Other(String),
}

impl std::str::FromStr for Purpose {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for Purpose {
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

// impl<'de> Deserialize<'de> for Purpose {
//     fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         utils::deserialize_using_from_str(d)
//     }
// }
