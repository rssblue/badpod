use crate::utils;
use crate::Other;
use strum::{EnumIter, EnumProperty};

/// Type of [Integrity](crate::podcast::Integrity).
#[derive(Debug, PartialEq, Eq, EnumProperty, EnumIter)]
pub enum IntegrityType {
    #[strum(props(str = "sri"))]
    Sri,
    #[strum(props(str = "pgp-signature"))]
    Pgp,

    Other(Other),
}

impl std::str::FromStr for IntegrityType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other((
                s.to_string(),
                "should be either \"sri\" or \"pgp-signature\"".to_string(),
            ))),
        }
    }
}

impl std::fmt::Display for IntegrityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other((s, _)) => write!(f, "{s}"),
            _ => match self.get_str("str") {
                Some(s) => write!(f, "{s}"),
                None => write!(f, "{self:?}"),
            },
        }
    }
}

impl IntegrityType {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(variant) => variant,
            Err(e) => Self::Other((s.to_string(), e)),
        }
    }
}
