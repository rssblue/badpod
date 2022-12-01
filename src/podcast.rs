use crate::basic;
use crate::itunes;
use crate::language::Language;
use crate::mime;
use crate::time::DateTime;

mod transcript;
pub use transcript::Rel as TranscriptRel;

mod person;
pub use person::Group as PersonGroup;
pub use person::Role as PersonRole;

mod location;
pub use location::{Geo, Osm, OsmType};

mod license;
pub use license::Type as LicenseType;

mod alternate_enclosure;
pub use alternate_enclosure::IntegrityType;

mod guid;
pub use guid::Guid;

mod medium;
pub use medium::Medium;

mod value;
pub use value::Method as ValueMethod;
pub use value::RecipientType as ValueRecipientType;
pub use value::Type as ValueType;

mod image;
pub use image::{Image, Images};

mod live_item;
pub use live_item::Status as LiveItemStatus;

mod social_interact;
pub use social_interact::Protocol as SocialProtocol;

mod block;
pub use block::Service;

mod txt;
pub use txt::Purpose as TxtPurpose;

/// A transcript or closed captions file.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Transcript {
    // #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    // #[serde(rename = "$attr:type")]
    pub type_: Option<mime::Transcript>,
    // #[serde(rename = "$attr:language")]
    pub language: Option<Language>,
    // #[serde(rename = "$attr:rel")]
    pub rel: Option<TranscriptRel>,
}

/// Indicates whether podcast hosting platforms are allowed to import the feed.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Locked {
    pub owner: Option<String>,
    pub value: Option<basic::Bool>,
}

/// Donation/funding link.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Funding {
    pub url: Option<String>,
    pub value: Option<String>,
}

/// Chapter data for an episode.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Chapters {
    // #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    // #[serde(rename = "$attr:type")]
    pub type_: Option<mime::Chapters>,
}

/// Soundbite of an episode.
#[derive(Debug, PartialEq, Default)]
pub struct Soundbite {
    // #[serde(rename = "$attr:startTime")]
    // #[serde_as(as = "Option<basic::FloatNonNegative>")]
    pub start_time: Option<basic::Float>,
    // #[serde(rename = "$attr:duration")]
    // #[serde_as(as = "Option<basic::FloatNonNegative>")]
    pub duration: Option<basic::Float>,
    // #[serde(rename = "$value")]
    pub value: Option<String>,
}

/// A person of interest to the podcast.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Person {
    // #[serde(rename = "$attr:group", default)]
    pub group: Option<PersonGroup>,
    // #[serde(rename = "$attr:role", default)]
    pub role: Option<PersonRole>,
    // #[serde(rename = "$attr:img")]
    pub img: Option<String>,
    // #[serde(rename = "$attr:href")]
    pub href: Option<String>,
    // #[serde(rename = "$value")]
    pub value: Option<String>,
}

/// Location of editorial focus for a podcast's content.
#[derive(Debug, PartialEq, Default)]
pub struct Location {
    // #[serde(rename = "$attr:geo")]
    pub geo: Option<Geo>,
    // #[serde(rename = "$attr:osm")]
    pub osm: Option<Osm>,
    // #[serde(rename = "$value")]
    pub value: Option<String>,
}

/// Indicates the season that a particular episode belongs to.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Season {
    // #[serde(rename = "$attr:name")]
    pub name: Option<String>,
    // #[serde(rename = "$value")]
    // #[serde_as(as = "Option<basic::IntegerNonNegative>")]
    pub value: Option<basic::Integer>,
}

/// Allows to specify episode number and how it should be displayed.
#[derive(Debug, PartialEq, Default)]
pub struct Episode {
    // #[serde(rename = "$attr:display")]
    pub display: Option<String>,
    // #[serde(rename = "$value")]
    // #[serde_as(as = "Option<basic::NumberNonNegative>")]
    pub value: Option<basic::Number>,
}

/// A trailer for the entire podcast or a specific season.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Trailer {
    // #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    // #[serde(rename = "$attr:pubdate", default)]
    pub pub_date: Option<DateTime>,
    // #[serde(rename = "$attr:length", default)]
    // #[serde_as(as = "Option<basic::IntegerNonNegative>")]
    pub length: Option<basic::Integer>,
    // #[serde(rename = "$attr:type")]
    pub type_: Option<mime::Enclosure>,
    // #[serde(rename = "$attr:season", default)]
    // #[serde_as(as = "Option<basic::IntegerNonNegative>")]
    pub season: Option<basic::Integer>,
    // #[serde(rename = "$value")]
    pub value: Option<String>,
}

/// License for a podcast or episode.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct License {
    // #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    // #[serde(rename = "$value")]
    pub value: Option<LicenseType>,
}

/// Different version of or a companion media to the file in [Enclosure](crate::Enclosure).
#[derive(Debug, PartialEq, Default)]
pub struct AlternateEnclosure {
    // #[serde(rename = "$attr:type", default)]
    pub type_: Option<mime::Enclosure>,
    // #[serde(rename = "$attr:length", default)]
    // #[serde_as(as = "Option<basic::IntegerNonNegative>")]
    pub length: Option<basic::Integer>,
    // #[serde(rename = "$attr:bitrate", default)]
    // #[serde_as(as = "Option<basic::FloatNonNegative>")]
    pub bit_rate: Option<basic::Float>,
    // #[serde(rename = "$attr:height", default)]
    // #[serde_as(as = "Option<basic::IntegerNonNegative>")]
    pub height: Option<basic::Integer>,
    // #[serde(rename = "$attr:lang", default)]
    pub language: Option<Language>,
    // #[serde(rename = "$attr:title", default)]
    pub title: Option<String>,
    // #[serde(rename = "$attr:rel", default)]
    pub rel: Option<String>,
    // TODO: this is quite complicated; will try to do later.
    // #[serde(rename = "$attr:codecs", default)]
    // pub codecs: Vec<String>,
    // #[serde(rename = "$attr:default", default)]
    pub default: Option<basic::Bool>,

    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:source"
    // )]
    pub podcast_source: Vec<Source>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:integrity"
    // )]
    pub podcast_integrity: Vec<Integrity>,
}

/// Location of a media file in [AlternateEnclosure](AlternateEnclosure).
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Source {
    // #[serde(rename = "$attr:contentType")]
    pub type_: Option<mime::Enclosure>,
    // #[serde(rename = "$attr:uri")]
    pub uri: Option<String>,
}

/// Method of verifying the integrity of the media in
/// [AlternateEnclosure](AlternateEnclosure).
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Integrity {
    // #[serde(rename = "$attr:type")]
    pub type_: Option<IntegrityType>,
    // #[serde(rename = "$attr:value")]
    pub value: Option<String>,
}

/// Describes cryptocurrency or payment layer used for transactions.
#[derive(Debug, PartialEq, Default)]
pub struct Value {
    pub type_: Option<ValueType>,
    pub method: Option<ValueMethod>,
    pub suggested: Option<basic::Float>,
    pub value_recipient: Vec<ValueRecipient>,
}

/// Destination for payments to be sent to during consumption of enclosed media.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct ValueRecipient {
    pub name: Option<String>,
    pub custom_key: Option<String>,
    pub custom_value: Option<String>,
    pub type_: Option<ValueRecipientType>,
    pub address: Option<String>,
    pub split: Option<basic::Integer>,
    pub fee: Option<basic::Bool>,
}

/// Used to deliver a live audio or video stream.
#[derive(Debug, PartialEq, Default)]
pub struct LiveItem {
    // #[serde(default)]
    pub description: Vec<String>,
    // #[serde(default)]
    pub link: Vec<String>,
    // #[serde(default)]
    pub title: Vec<String>,
    // #[serde(default)]
    pub enclosure: Vec<crate::Enclosure>,
    // #[serde(default)]
    pub guid: Vec<crate::Guid>,
    // #[serde(default, rename = "pubDate")]
    pub pub_date: Vec<DateTime>,

    // #[serde(rename = "$attr:status", default)]
    pub status: Option<LiveItemStatus>,
    // #[serde(rename = "$attr:start", default)]
    // #[serde_as(as = "Option<crate::time::DatetimeISO8601>")]
    pub start: Option<DateTime>,
    // #[serde(rename = "$attr:end", default)]
    // #[serde_as(as = "Option<crate::time::DatetimeISO8601>")]
    pub end: Option<DateTime>,

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
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:duration",
    //     default
    // )]
    // #[serde_as(as = "Vec<basic::NumberNonNegative>")]
    pub itunes_duration: Vec<basic::Number>,
    // #[serde(
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:season",
    //     default
    // )]
    // #[serde_as(as = "Vec<basic::IntegerPositive>")]
    pub itunes_season: Vec<basic::Integer>,
    // #[serde(
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:episode",
    //     default
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
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:transcript",
    //     default
    // )]
    pub podcast_transcript: Vec<Transcript>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:chapters"
    // )]
    pub podcast_chapters: Vec<Chapters>,
    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:soundbite",
    //     default
    // )]
    pub podcast_soundbite: Vec<Soundbite>,
    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:person",
    //     default
    // )]
    pub podcast_person: Vec<Person>,
    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:location",
    //     default
    // )]
    pub podcast_location: Vec<Location>,
    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:season",
    //     default
    // )]
    pub podcast_season: Vec<Season>,
    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:episode",
    //     default
    // )]
    pub podcast_episode: Vec<Episode>,
    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:alternateEnclosure",
    //     default
    // )]
    pub podcast_alternate_enclosure: Vec<AlternateEnclosure>,
    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:value",
    //     default
    // )]
    pub podcast_value: Vec<Value>,
    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:images",
    //     default
    // )]
    pub podcast_images: Vec<Images>,

    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:contentLink",
    //     default
    // )]
    pub content_link: Vec<ContentLink>,
    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:socialInteract",
    //     default
    // )]
    pub podcast_social_interact: Vec<SocialInteract>,
    // #[serde(
    //     default,
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:license"
    // )]
    pub podcast_license: Vec<License>,
    // #[serde(
    //     rename = "{https://podcastindex.org/namespace/1.0}podcast:txt",
    //     default
    // )]
    pub podcast_txt: Vec<Txt>,
}

/// Used to indicate that the content being delivered by [LiveItem](LiveItem) can be found at an external location.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct ContentLink {
    // #[serde(rename = "$attr:href", default)]
    pub href: Option<String>,
    // #[serde(rename = "$value")]
    pub value: Option<String>,
}

/// Allows a podcaster to attach the URL of a "root post" of a comment thread to an episode.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct SocialInteract {
    // #[serde(rename = "$attr:uri", default)]
    pub uri: Option<String>,
    // #[serde(rename = "$attr:protocol", default)]
    pub protocol: Option<SocialProtocol>,
    // #[serde(rename = "$attr:accountId", default)]
    pub account_id: Option<String>,
    // #[serde(rename = "$attr:accountUrl", default)]
    pub account_url: Option<String>,
    // #[serde(rename = "$attr:priority", default)]
    pub priority: Option<basic::Integer>,
}

/// Allows a podcaster to express which platforms are allowed to publicly display this feed and its contents.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Block {
    // #[serde(rename = "$attr:id", default)]
    pub id: Option<Service>,
    // #[serde(rename = "$value")]
    // #[serde_as(as = "Option<basic::BoolYN>")]
    pub value: Option<basic::Bool>,
}

/// Free-form text to allow for uses that might be niche or otherwise not rise to the level of needing a dedicated tag.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Txt {
    // #[serde(rename = "$attr:purpose", default)]
    pub purpose: Option<TxtPurpose>,
    // #[serde(rename = "$value")]
    pub value: Option<String>,
}
