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
    pub url: Option<String>,
    pub type_: Option<mime::Transcript>,
    pub language: Option<Language>,
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
    pub url: Option<String>,
    pub type_: Option<mime::Chapters>,
}

/// Soundbite of an episode.
#[derive(Debug, PartialEq, Default)]
pub struct Soundbite {
    pub start_time: Option<basic::Float>,
    pub duration: Option<basic::Float>,
    pub value: Option<String>,
}

/// A person of interest to the podcast.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Person {
    pub group: Option<PersonGroup>,
    pub role: Option<PersonRole>,
    pub img: Option<String>,
    pub href: Option<String>,
    pub value: Option<String>,
}

/// Location of editorial focus for a podcast's content.
#[derive(Debug, PartialEq, Default)]
pub struct Location {
    pub geo: Option<Geo>,
    pub osm: Option<Osm>,
    pub value: Option<String>,
}

/// Indicates the season that a particular episode belongs to.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Season {
    pub name: Option<String>,
    pub value: Option<basic::Integer>,
}

/// Allows to specify episode number and how it should be displayed.
#[derive(Debug, PartialEq, Default)]
pub struct Episode {
    pub display: Option<String>,
    pub value: Option<basic::Number>,
}

/// A trailer for the entire podcast or a specific season.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Trailer {
    pub url: Option<String>,
    pub pub_date: Option<DateTime>,
    pub length: Option<basic::Integer>,
    pub type_: Option<mime::Enclosure>,
    pub season: Option<basic::Integer>,
    pub value: Option<String>,
}

/// License for a podcast or episode.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct License {
    pub url: Option<String>,
    pub value: Option<LicenseType>,
}

/// Different version of or a companion media to the file in [Enclosure](crate::Enclosure).
#[derive(Debug, PartialEq, Default)]
pub struct AlternateEnclosure {
    pub type_: Option<mime::Enclosure>,
    pub length: Option<basic::Integer>,
    pub bit_rate: Option<basic::Float>,
    pub height: Option<basic::Integer>,
    pub language: Option<Language>,
    pub title: Option<String>,
    pub rel: Option<String>,
    pub default: Option<basic::Bool>,

    pub podcast_source: Vec<Source>,
    pub podcast_integrity: Vec<Integrity>,
}

/// Location of a media file in [AlternateEnclosure](AlternateEnclosure).
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Source {
    pub type_: Option<mime::Enclosure>,
    pub uri: Option<String>,
}

/// Method of verifying the integrity of the media in
/// [AlternateEnclosure](AlternateEnclosure).
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Integrity {
    pub type_: Option<IntegrityType>,
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
    pub status: Option<LiveItemStatus>,
    pub start: Option<DateTime>,
    pub end: Option<DateTime>,

    pub description: Vec<String>,
    pub link: Vec<String>,
    pub title: Vec<String>,
    pub enclosure: Vec<crate::Enclosure>,
    pub guid: Vec<crate::Guid>,
    pub pub_date: Vec<DateTime>,

    pub content_encoded: Vec<String>,

    pub itunes_block: Vec<itunes::Yes>,
    pub itunes_duration: Vec<basic::Number>,
    pub itunes_season: Vec<basic::Integer>,
    pub itunes_episode: Vec<basic::Integer>,
    pub itunes_explicit: Vec<basic::Bool>,
    pub itunes_image: Vec<itunes::Image>,
    pub itunes_title: Vec<String>,
    pub itunes_type: Vec<itunes::EpisodeType>,

    pub podcast_transcript: Vec<Transcript>,
    pub podcast_chapters: Vec<Chapters>,
    pub podcast_soundbite: Vec<Soundbite>,
    pub podcast_person: Vec<Person>,
    pub podcast_location: Vec<Location>,
    pub podcast_season: Vec<Season>,
    pub podcast_episode: Vec<Episode>,
    pub podcast_alternate_enclosure: Vec<AlternateEnclosure>,
    pub podcast_value: Vec<Value>,
    pub podcast_images: Vec<Images>,
    pub podcast_social_interact: Vec<SocialInteract>,
    pub podcast_license: Vec<License>,
    pub podcast_txt: Vec<Txt>,

    pub podcast_content_link: Vec<ContentLink>,
}

/// Used to indicate that the content being delivered by [LiveItem](LiveItem) can be found at an external location.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct ContentLink {
    pub href: Option<String>,
    pub value: Option<String>,
}

/// Allows a podcaster to attach the URL of a "root post" of a comment thread to an episode.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct SocialInteract {
    pub uri: Option<String>,
    pub protocol: Option<SocialProtocol>,
    pub account_id: Option<String>,
    pub account_url: Option<String>,
    pub priority: Option<basic::Integer>,
}

/// Allows a podcaster to express which platforms are allowed to publicly display this feed and its contents.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Block {
    pub id: Option<Service>,
    pub value: Option<basic::Bool>,
}

/// Free-form text to allow for uses that might be niche or otherwise not rise to the level of needing a dedicated tag.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Txt {
    pub purpose: Option<TxtPurpose>,
    pub value: Option<String>,
}
