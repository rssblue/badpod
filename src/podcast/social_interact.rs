use crate::utils;
use crate::Other;
use strum_macros::EnumIter;

/// Social protocols that can be used in [SocialInteract](crate::podcast::SocialInteract).
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Protocol {
    Disabled,
    ActivityPub,
    Twitter,
    Lightning,

    Other(Other),
}

impl std::str::FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other((s.to_string(), "should be one of the protocols at <https://raw.githubusercontent.com/Podcastindex-org/podcast-namespace/main/socialprotocols.txt>".to_string()))),
        }
    }
}

impl std::fmt::Display for Protocol {
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

impl Protocol {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(protocol) => protocol,
            Err(e) => Self::Other((s.to_string(), e)),
        }
    }
}
