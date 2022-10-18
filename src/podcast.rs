use serde::Deserialize;

use crate::basic;
use crate::itunes;
use crate::language::Language;
use crate::mimetype;
pub use crate::time::DateTime;

mod transcript;
pub use transcript::TranscriptRel;

mod chapters;
pub use chapters::ChaptersType;

mod person;
pub use person::{PersonGroup, PersonRole};

mod location;
pub use location::{Geo, GeoCoordinates, OSMObject, OSMType, OSM};

mod license;
pub use license::LicenseType;

mod alternate_enclosure;
pub use alternate_enclosure::IntegrityType;

mod guid;
pub use guid::GUID;

mod medium;
pub use medium::Medium;

mod value;
pub use value::{ValueMethod, ValueRecipientType, ValueType};

mod image;
pub use image::{Image, Images};

mod live_item;
pub use live_item::LiveItemStatus;

mod social_interact;
pub use social_interact::SocialProtocol;

mod block;
pub use block::Service;

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Transcript {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$attr:type")]
    pub type_: Option<mimetype::Transcript>,
    #[serde(rename = "$attr:language")]
    pub language: Option<Language>,
    #[serde(rename = "$attr:rel")]
    pub rel: Option<TranscriptRel>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Locked {
    #[serde(rename = "$attr:owner")]
    pub owner: Option<String>,
    #[serde(rename = "$value", deserialize_with = "basic::option_bool_yn")]
    pub value: Option<basic::Bool>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Funding {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Chapters {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$attr:type")]
    pub type_: Option<ChaptersType>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Soundbite {
    #[serde(
        rename = "$attr:startTime",
        deserialize_with = "basic::option_float_nonnegative"
    )]
    pub start_time: Option<basic::Float>,
    #[serde(
        rename = "$attr:duration",
        deserialize_with = "basic::option_float_nonnegative"
    )]
    pub duration: Option<basic::Float>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Person {
    #[serde(
        rename = "$attr:group",
        deserialize_with = "person::option_person_group",
        default
    )]
    pub group: Option<PersonGroup>,
    #[serde(
        rename = "$attr:role",
        deserialize_with = "person::option_person_role",
        default
    )]
    pub role: Option<PersonRole>,
    #[serde(rename = "$attr:img")]
    pub img: Option<String>,
    #[serde(rename = "$attr:href")]
    pub href: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Location {
    #[serde(rename = "$attr:geo")]
    pub geo: Option<Geo>,
    #[serde(rename = "$attr:osm")]
    pub osm: Option<OSM>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Season {
    #[serde(rename = "$attr:name")]
    pub name: Option<String>,
    #[serde(
        rename = "$value",
        deserialize_with = "basic::option_integer_nonnegative"
    )]
    pub value: Option<basic::Integer>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Episode {
    #[serde(rename = "$attr:display")]
    pub display: Option<String>,
    #[serde(
        rename = "$value",
        deserialize_with = "basic::option_number_nonnegative"
    )]
    pub value: Option<basic::Number>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Trailer {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$attr:pubdate", default)]
    pub pub_date: Option<DateTime>,
    #[serde(
        rename = "$attr:length",
        deserialize_with = "basic::option_integer_nonnegative",
        default
    )]
    pub length: Option<basic::Integer>,
    #[serde(rename = "$attr:type")]
    pub type_: Option<mimetype::Enclosure>,
    #[serde(
        rename = "$attr:season",
        deserialize_with = "basic::option_integer_nonnegative",
        default
    )]
    pub season: Option<basic::Integer>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct License {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<LicenseType>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct AlternateEnclosure {
    #[serde(rename = "$attr:type", default)]
    pub type_: Option<mimetype::Enclosure>,
    #[serde(
        rename = "$attr:length",
        deserialize_with = "basic::option_integer_nonnegative",
        default
    )]
    pub length: Option<basic::Integer>,
    #[serde(
        rename = "$attr:bitrate",
        deserialize_with = "basic::option_float_nonnegative",
        default
    )]
    pub bit_rate: Option<basic::Float>,
    #[serde(
        rename = "$attr:height",
        deserialize_with = "basic::option_integer_nonnegative",
        default
    )]
    pub height: Option<basic::Integer>,
    #[serde(rename = "$attr:lang", default)]
    pub language: Option<Language>,
    #[serde(rename = "$attr:title", default)]
    pub title: Option<String>,
    #[serde(rename = "$attr:rel", default)]
    pub rel: Option<String>,
    // TODO: this is quite complicated; will try to do later.
    // #[serde(rename = "$attr:codecs", default)]
    // pub codecs: Vec<String>,
    #[serde(rename = "$attr:default", default)]
    pub default: Option<basic::Bool>,

    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:source",
        default
    )]
    pub podcast_sources: Vec<Source>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:integrity",
        default
    )]
    pub podcast_integrity: Option<Integrity>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Source {
    #[serde(rename = "$attr:contentType")]
    pub type_: Option<mimetype::Enclosure>,
    #[serde(rename = "$attr:uri")]
    pub uri: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Integrity {
    #[serde(rename = "$attr:type")]
    pub type_: Option<IntegrityType>,
    #[serde(rename = "$attr:value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Value {
    #[serde(rename = "$attr:type", default)]
    pub type_: Option<ValueType>,
    #[serde(rename = "$attr:method", default)]
    pub method: Option<ValueMethod>,
    #[serde(
        rename = "$attr:suggested",
        deserialize_with = "basic::option_float_positive",
        default
    )]
    pub suggested: Option<basic::Float>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:valueRecipient",
        default
    )]
    pub value_recipients: Vec<ValueRecipient>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct ValueRecipient {
    #[serde(rename = "$attr:name")]
    pub name: Option<String>,
    #[serde(rename = "$attr:customKey")]
    pub custom_key: Option<String>,
    #[serde(rename = "$attr:customValue")]
    pub custom_value: Option<String>,
    #[serde(rename = "$attr:type", default)]
    pub type_: Option<ValueRecipientType>,
    #[serde(rename = "$attr:address")]
    pub address: Option<String>,
    #[serde(
        rename = "$attr:split",
        deserialize_with = "basic::option_integer_positive",
        default
    )]
    pub split: Option<basic::Integer>,
    #[serde(rename = "$attr:fee", default)]
    pub fee: Option<basic::Bool>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct LiveItem {
    pub description: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
    pub enclosure: Option<crate::Enclosure>,
    pub guid: Option<crate::GUID>,
    #[serde(default, rename = "pubDate")]
    pub pub_date: Option<DateTime>,

    #[serde(rename = "$attr:status", default)]
    pub status: Option<LiveItemStatus>,
    #[serde(
        rename = "$attr:start",
        deserialize_with = "crate::time::option_datetime_iso8601",
        default
    )]
    pub start: Option<DateTime>,
    #[serde(
        rename = "$attr:end",
        deserialize_with = "crate::time::option_datetime_iso8601",
        default
    )]
    pub end: Option<DateTime>,

    #[serde(rename = "{http://purl.org/rss/1.0/modules/content/}content:encoded")]
    pub content_encoded: Option<String>,

    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:block")]
    pub itunes_block: Option<itunes::Yes>,
    #[serde(
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:duration",
        deserialize_with = "basic::option_number_nonnegative",
        default
    )]
    pub itunes_duration: Option<basic::Number>,
    #[serde(
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:season",
        deserialize_with = "basic::option_integer_positive",
        default
    )]
    pub itunes_season: Option<basic::Integer>,
    #[serde(
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:episode",
        deserialize_with = "basic::option_integer_positive",
        default
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
    pub podcast_transcripts: Vec<Transcript>,
    #[serde(rename = "{https://podcastindex.org/namespace/1.0}podcast:chapters")]
    pub podcast_chapters: Option<Chapters>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:soundbite",
        default
    )]
    pub podcast_soundbites: Vec<Soundbite>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:person",
        default
    )]
    pub podcast_persons: Vec<Person>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:location",
        default
    )]
    pub podcast_location: Option<Location>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:season",
        default
    )]
    pub podcast_season: Option<Season>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:episode",
        default
    )]
    pub podcast_episode: Option<Episode>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:alternateEnclosure",
        default
    )]
    pub podcast_alternate_enclosures: Vec<AlternateEnclosure>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:value",
        default
    )]
    pub podcast_value: Option<Value>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:images",
        default
    )]
    pub podcast_images: Option<Images>,

    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:contentLink",
        default
    )]
    pub content_links: Vec<ContentLink>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:socialInteract",
        default
    )]
    pub podcast_social_interacts: Vec<SocialInteract>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct ContentLink {
    #[serde(rename = "$attr:href", default)]
    pub href: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct SocialInteract {
    #[serde(rename = "$attr:uri", default)]
    pub uri: Option<String>,
    #[serde(rename = "$attr:protocol", default)]
    pub protocol: Option<SocialProtocol>,
    #[serde(rename = "$attr:accountId", default)]
    pub account_id: Option<String>,
    #[serde(rename = "$attr:accountUrl", default)]
    pub account_url: Option<String>,
    #[serde(rename = "$attr:priority", default)]
    pub priority: Option<basic::Integer>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Block {
    #[serde(rename = "$attr:id", default)]
    pub id: Option<Service>,
    #[serde(rename = "$value", deserialize_with = "basic::option_bool_yn")]
    pub value: Option<basic::Bool>,
}
