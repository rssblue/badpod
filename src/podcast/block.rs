use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Service slugs taken from
/// <https://raw.githubusercontent.com/Podcastindex-org/podcast-namespace/main/serviceslugs.txt>.
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
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
    #[strum(serialize = "gpodder")]
    GPodder,
    #[strum(serialize = "hypercatcher")]
    HyperCatcher,
    Kasts,
    Libsyn,
    Mastodon,
    Megafono,
    Megaphone,
    #[strum(serialize = "omnystudio")]
    OmnyStudio,
    Overcast,
    #[strum(serialize = "paypal")]
    PayPal,
    Pinecast,
    Podbean,
    #[strum(serialize = "podcastaddict")]
    PodcastAddict,
    #[strum(serialize = "podcastguru")]
    PodcastGuru,
    #[strum(serialize = "podcastindex")]
    PodcastIndex,
    Podcasts,
    Podchaser,
    #[strum(serialize = "podcloud")]
    PodCloud,
    Podfriend,
    Podiant,
    Podigee,
    Podnews,
    Podomatic,
    #[strum(serialize = "podserve")]
    PodServe,
    Podverse,
    #[strum(serialize = "redcircle")]
    RedCircle,
    Relay,
    #[strum(serialize = "resonaterecordings")]
    ResonateRecordings,
    Rss,
    #[strum(serialize = "shoutengine")]
    ShoutEngine,
    Simplecast,
    Slack,
    #[strum(serialize = "soundcloud")]
    SoundCloud,
    Spotify,
    Spreaker,
    #[strum(serialize = "tiktok")]
    TikTok,
    Transistor,
    Twitter,
    Whooshkaa,
    #[strum(serialize = "youtube")]
    YouTube,
    #[strum(serialize = "zencast")]
    ZenCast,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for Service {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match Self::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(Self::Other(s)),
        }
    }
}
