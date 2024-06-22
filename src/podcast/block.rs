use crate::utils;
use crate::Other;
use strum::EnumIter;

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

    Other(Other),
}

impl std::str::FromStr for Service {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other((s.to_string(), "should be a service slug from <https://raw.githubusercontent.com/Podcastindex-org/podcast-namespace/main/serviceslugs.txt>".to_string()))),
        }
    }
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other((s, _)) => write!(f, "{s}"),
            _ => {
                let s = format!("{self:?}");
                write!(f, "{}", s.to_lowercase())
            }
        }
    }
}

impl Service {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(service) => service,
            Err(e) => Self::Other((s.to_string(), e)),
        }
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
            format!("{}", Service::Other(("other-service".to_string(), "should be a service slug from <https://raw.githubusercontent.com/Podcastindex-org/podcast-namespace/main/serviceslugs.txt>".to_string()))),
            "other-service"
        );
    }
}
