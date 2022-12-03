use crate::Other;
use regex::Regex;

/// Global unique identifier for a podcast.
#[derive(Debug, PartialEq, Eq)]
pub enum Guid {
    Ok(String),
    Other(Other),
}

impl std::str::FromStr for Guid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const GUID_REGEX: &str =
            r"^[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}$";
        let re = match Regex::new(GUID_REGEX) {
            Ok(re) => re,
            Err(e) => return Err(e.to_string()),
        };

        if !re.is_match(s) {
            return Ok(Self::Other((
                s.to_string(),
                format!("should be a UUIDv5 matching regular expression \"{GUID_REGEX}\""),
            )));
        }

        Ok(Self::Ok(s.to_string()))
    }
}

impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other((s, _)) => write!(f, "{s}"),
        }
    }
}

impl Guid {
    pub fn parse(s: &str) -> Self {
        match s.parse::<Self>() {
            Ok(guid) => guid,
            Err(e) => Self::Other((s.to_string(), e)),
        }
    }
}
