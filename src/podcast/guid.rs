use regex::Regex;

/// Global unique identifier for a podcast.
#[derive(Debug, PartialEq, Eq)]
pub enum Guid {
    Ok(String),
    Other(String),
}

impl std::str::FromStr for Guid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = match Regex::new(
            r"^[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}$",
        ) {
            Ok(re) => re,
            Err(e) => return Err(e.to_string()),
        };

        if !re.is_match(s) {
            return Ok(Self::Other(s.to_string()));
        }

        Ok(Self::Ok(s.to_string()))
    }
}

impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl Guid {
    pub fn parse(s: &str) -> Self {
        match s.parse::<Self>() {
            Ok(guid) => guid,
            Err(_) => Self::Other(s.to_string()),
        }
    }
}
