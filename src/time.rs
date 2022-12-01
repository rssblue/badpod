pub enum TimeFormat {
    Rfc2822,
    Iso8601,
}

/// Used for deserializing combined dates and times.
#[derive(Debug, PartialEq, Eq)]
pub enum DateTime {
    Ok(chrono::DateTime<chrono::FixedOffset>),
    Other(String),
}

impl DateTime {
    pub fn parse(s: &str, format: TimeFormat) -> Self {
        match format {
            TimeFormat::Rfc2822 => match chrono::DateTime::parse_from_rfc2822(s) {
                Ok(dt) => DateTime::Ok(dt),
                Err(_) => DateTime::Other(s.to_string()),
            },
            TimeFormat::Iso8601 => {
                if let Ok(t) = chrono::DateTime::parse_from_rfc3339(&s) {
                    return DateTime::Ok(t);
                }

                if let Ok(t) = chrono::DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f%:z") {
                    return DateTime::Ok(t);
                }

                DateTime::Other(s.to_string())
            }
        }
    }
}

impl std::str::FromStr for DateTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match chrono::DateTime::parse_from_rfc2822(s) {
            Ok(t) => Ok(Self::Ok(t)),
            Err(_) => Ok(DateTime::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{t}"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}
