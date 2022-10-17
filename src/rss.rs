use serde::Deserialize;

use crate::itunes;
use crate::podcast;

use crate::basic;
use crate::language;
use crate::mimetype;
use crate::time;

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct XML {
    pub rss: RSS,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct RSS {
    #[serde(rename = "$attr:version")]
    pub version: Option<String>,

    pub channel: Option<Channel>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Channel {
    pub copyright: Option<String>,
    pub description: Option<String>,
    pub generator: Option<String>,
    pub language: Option<language::Language>,
    pub link: Option<String>,
    pub title: Option<String>,

    #[serde(rename = "{http://purl.org/rss/1.0/modules/content/}content:encoded")]
    pub content_encoded: Option<String>,

    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:author")]
    pub itunes_author: Option<String>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:block")]
    pub itunes_block: Option<itunes::Yes>,
    #[serde(
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:category",
        default
    )]
    pub itunes_categories: Vec<itunes::Category>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:complete")]
    pub itunes_complete: Option<itunes::Yes>,
    #[serde(
        default,
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:explicit"
    )]
    pub itunes_explicit: Option<basic::Bool>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:image")]
    pub itunes_image: Option<itunes::Image>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:new-feed-url")]
    pub itunes_new_feed_url: Option<String>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:owner")]
    pub itunes_owner: Option<itunes::Owner>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:type")]
    pub itunes_type: Option<itunes::PodcastType>,
    #[serde(
        default,
        rename = "{https://podcastindex.org/namespace/1.0}podcast:locked"
    )]
    pub podcast_locked: Option<podcast::Locked>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:funding",
        default
    )]
    pub podcast_fundings: Vec<podcast::Funding>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:person",
        default
    )]
    pub podcast_persons: Vec<podcast::Person>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:location",
        default
    )]
    pub podcast_location: Option<podcast::Location>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:trailer",
        default
    )]
    pub podcast_trailers: Vec<podcast::Trailer>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:license",
        default
    )]
    pub podcast_license: Option<podcast::License>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:guid",
        default
    )]
    pub podcast_guid: Option<podcast::GUID>,

    #[serde(rename = "item", default)]
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Item {
    pub description: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
    pub enclosure: Option<Enclosure>,
    pub guid: Option<GUID>,
    #[serde(default, deserialize_with = "time::option_datefmt", rename = "pubDate")]
    pub pub_date: Option<time::DateTime>,

    #[serde(rename = "{http://purl.org/rss/1.0/modules/content/}content:encoded")]
    pub content_encoded: Option<String>,

    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:block")]
    pub itunes_block: Option<itunes::Yes>,
    #[serde(
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:duration",
        deserialize_with = "basic::option_number_nonnegative"
    )]
    pub itunes_duration: Option<basic::Number>,
    #[serde(
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:season",
        deserialize_with = "basic::option_integer_positive"
    )]
    pub itunes_season: Option<basic::Integer>,
    #[serde(
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:episode",
        deserialize_with = "basic::option_integer_positive"
    )]
    pub itunes_episode: Option<basic::Integer>,
    #[serde(
        default,
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:explicit"
    )]
    pub itunes_explicit: Option<basic::Bool>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:image")]
    pub itunes_image: Option<itunes::Image>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:title")]
    pub itunes_title: Option<String>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:episodeType")]
    pub itunes_type: Option<itunes::EpisodeType>,

    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:transcript",
        default
    )]
    pub podcast_transcripts: Vec<podcast::Transcript>,
    #[serde(rename = "{https://podcastindex.org/namespace/1.0}podcast:chapters")]
    pub podcast_chapters: Option<podcast::Chapters>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:soundbite",
        default
    )]
    pub podcast_soundbites: Vec<podcast::Soundbite>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:person",
        default
    )]
    pub podcast_persons: Vec<podcast::Person>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:location",
        default
    )]
    pub podcast_location: Option<podcast::Location>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:season",
        default
    )]
    pub podcast_season: Option<podcast::Season>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:episode",
        default
    )]
    pub podcast_episode: Option<podcast::Episode>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:alternateEnclosure",
        default
    )]
    pub podcast_alternate_enclosures: Vec<podcast::AlternateEnclosure>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Enclosure {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$attr:length")]
    pub length: Option<usize>,
    #[serde(rename = "$attr:type")]
    pub type_: Option<mimetype::Enclosure>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct GUID {
    #[serde(rename = "$attr:isPermaLink")]
    pub is_permalink: Option<basic::Bool>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}
