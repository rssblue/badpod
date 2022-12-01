use crate::itunes;
use crate::podcast;

use crate::basic;
use crate::language;
use crate::mime;
use crate::parse_tree;
use crate::time;

/// Converts contents of an XML file of podcast's RSS feed to [Rss](Rss) struct.
pub fn from_str(feed_str: &str) -> Result<Rss, parse_tree::Error> {
    parse_tree::parse(feed_str)
}

#[derive(Debug, PartialEq, Default)]
pub struct Xml {
    pub rss: Rss,
}

/// RSS feed.
#[derive(Debug, PartialEq, Default)]
pub struct Rss {
    pub version: Option<String>,
    pub channel: Vec<Channel>,
}

/// Podcast feed.
#[derive(Debug, PartialEq, Default)]
pub struct Channel {
    pub category: Vec<String>,
    pub copyright: Vec<String>,
    pub description: Vec<String>,
    pub generator: Vec<String>,
    pub language: Vec<language::Language>,
    pub last_build_date: Vec<time::DateTime>,
    pub link: Vec<String>,
    pub managing_editor: Vec<String>,
    pub pub_date: Vec<time::DateTime>,
    pub title: Vec<String>,
    pub ttl: Vec<basic::Integer>,
    pub web_master: Vec<String>,

    pub content_encoded: Vec<String>,

    pub itunes_author: Vec<String>,
    pub itunes_block: Vec<itunes::Yes>,
    pub itunes_category: Vec<itunes::Category>,
    pub itunes_complete: Vec<itunes::Yes>,
    pub itunes_explicit: Vec<basic::Bool>,
    pub itunes_image: Vec<itunes::Image>,
    pub itunes_new_feed_url: Vec<String>,
    pub itunes_owner: Vec<itunes::Owner>,
    pub itunes_type: Vec<itunes::PodcastType>,

    pub podcast_locked: Vec<podcast::Locked>,
    pub podcast_funding: Vec<podcast::Funding>,
    pub podcast_person: Vec<podcast::Person>,
    pub podcast_location: Vec<podcast::Location>,
    pub podcast_trailer: Vec<podcast::Trailer>,
    pub podcast_license: Vec<podcast::License>,
    pub podcast_guid: Vec<podcast::Guid>,
    pub podcast_value: Vec<podcast::Value>,
    pub podcast_medium: Vec<podcast::Medium>,
    pub podcast_images: Vec<podcast::Images>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:liveItem"
    // )]
    pub podcast_live_item: Vec<podcast::LiveItem>,
    pub podcast_block: Vec<podcast::Block>,
    pub podcast_txt: Vec<podcast::Txt>,

    // #[serde(default)]
    pub item: Vec<Item>,
}

/// Podcast episode.
#[derive(Debug, PartialEq, Default)]
pub struct Item {
    // #[serde(default)]
    pub description: Vec<String>,
    // #[serde(default)]
    pub link: Vec<String>,
    // #[serde(default)]
    pub title: Vec<String>,
    // #[serde(default)]
    pub enclosure: Vec<Enclosure>,
    // #[serde(default)]
    pub guid: Vec<Guid>,
    // #[serde(default, rename = "pubDate")]
    pub pub_date: Vec<time::DateTime>,

    // #[serde(
    //     default,
    //     rename = "{http://purl.org/rss/1.0/modules/content/}content:encoded"
    // )]
    pub content_encoded: Vec<String>,

    // #[serde(
    //     default,
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:block"
    // )]
    pub itunes_block: Vec<itunes::Yes>,
    // #[serde(
    //     default,
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:duration"
    // )]
    // #[serde_as(as = "Vec<basic::NumberNonNegative>")]
    pub itunes_duration: Vec<basic::Number>,
    // #[serde(
    //     default,
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:season"
    // )]
    // #[serde_as(as = "Vec<basic::IntegerPositive>")]
    pub itunes_season: Vec<basic::Integer>,
    // #[serde(
    //     default,
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:episode"
    // )]
    // #[serde_as(as = "Vec<basic::IntegerPositive>")]
    pub itunes_episode: Vec<basic::Integer>,
    // #[serde(
    //     default,
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:explicit"
    // )]
    pub itunes_explicit: Vec<basic::Bool>,
    // #[serde(
    //     default,
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:image"
    // )]
    pub itunes_image: Vec<itunes::Image>,
    // #[serde(
    //     default,
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:title"
    // )]
    pub itunes_title: Vec<String>,
    // #[serde(
    //     default,
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:episodeType"
    // )]
    pub itunes_type: Vec<itunes::EpisodeType>,

    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:transcript"
    // )]
    pub podcast_transcript: Vec<podcast::Transcript>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:chapters"
    // )]
    pub podcast_chapters: Vec<podcast::Chapters>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:soundbite"
    // )]
    pub podcast_soundbite: Vec<podcast::Soundbite>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:person"
    // )]
    pub podcast_person: Vec<podcast::Person>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:location"
    // )]
    pub podcast_location: Vec<podcast::Location>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:season"
    // )]
    pub podcast_season: Vec<podcast::Season>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:episode"
    // )]
    pub podcast_episode: Vec<podcast::Episode>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:alternateEnclosure"
    // )]
    pub podcast_alternate_enclosure: Vec<podcast::AlternateEnclosure>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:value"
    // )]
    pub podcast_value: Vec<podcast::Value>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:images"
    // )]
    pub podcast_images: Vec<podcast::Images>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:socialInteract"
    // )]
    pub podcast_social_interact: Vec<podcast::SocialInteract>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:license"
    // )]
    pub podcast_license: Vec<podcast::License>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:txt"
    // )]
    pub podcast_txt: Vec<podcast::Txt>,
}

/// Episode's media content.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Enclosure {
    // #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    // #[serde(rename = "$attr:length")]
    pub length: Option<basic::Integer>,
    // #[serde(rename = "$attr:type")]
    pub type_: Option<mime::Enclosure>,
}

/// Episode's globally unique identifier.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Guid {
    // #[serde(rename = "$attr:isPermaLink")]
    pub is_permalink: Option<basic::Bool>,
    // #[serde(rename = "$value")]
    pub value: Option<String>,
}
