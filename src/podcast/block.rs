use crate::utils;
use serde::{Deserialize, Deserializer};
use std::fmt;
use strum_macros::EnumIter;

/// Podcast platforms.
///
/// Taken from
/// <https://raw.githubusercontent.com/Podcastindex-org/podcast-namespace/main/serviceslugs.txt>.
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Service {
    Acast,
    Amazon,
    Anchor,
    Apple,
    Audible,
    Audioboom,
    Backtracks,
    Bitcoin,
    Blubrry,
    Buzzsprout,
    Captivate,
    Castos,
    Castopod,
    Facebook,
    Fireside,
    Fyyd,
    Google,
    GPodder,
    HyperCatcher,
    Kasts,
    Libsyn,
    Mastodon,
    Megafono,
    Megaphone,
    OmnyStudio,
    Overcast,
    PayPal,
    Pinecast,
    Podbean,
    PodcastAddict,
    PodcastGuru,
    PodcastIndex,
    Podcasts,
    Podchaser,
    PodCloud,
    Podfriend,
    Podiant,
    Podigee,
    Podnews,
    Podomatic,
    PodServe,
    Podverse,
    RedCircle,
    Relay,
    ResonateRecordings,
    Rss,
    ShoutEngine,
    Simplecast,
    Slack,
    SoundCloud,
    Spotify,
    Spreaker,
    TikTok,
    Transistor,
    Twitter,
    Whooshkaa,
    YouTube,
    ZenCast,

    Other(String),
}

impl std::str::FromStr for Service {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => {
                let s = format!("{:?}", self);
                write!(f, "{}", s.to_lowercase())
            }
        }
    }
}

impl<'de> Deserialize<'de> for Service {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        utils::deserialize_using_from_str(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        pretty_assertions::assert_eq!(format!("{}", Service::YouTube), "youtube");
        pretty_assertions::assert_eq!(format!("{}", Service::Amazon), "amazon");
        pretty_assertions::assert_eq!(format!("{}", Service::TikTok), "tiktok");
        pretty_assertions::assert_eq!(
            format!("{}", Service::Other("other-service".to_string())),
            "other-service"
        );
    }
}
