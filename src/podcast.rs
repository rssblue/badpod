use serde::Deserialize;

use crate::basic;
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
    #[serde(rename = "$attr:geo", deserialize_with = "location::option_geo")]
    pub geo: Option<Geo>,
    #[serde(rename = "$attr:osm", deserialize_with = "location::option_osm")]
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
    #[serde(
        rename = "$attr:pubdate",
        deserialize_with = "crate::time::option_datefmt",
        default
    )]
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
