use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

mod category;
pub use category::{CategoryName, SubcategoryName};

/// Apple Podcasts podcast category.
#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Category {
    #[serde(rename = "$attr:text")]
    pub text: Option<CategoryName>,

    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:category")]
    pub subcategory: Option<Subcategory>,
}

/// Apple Podcasts podcast subcategory.
///
/// Just because a subcategory is deserialized does not mean that it is compatible with the
/// category in the parent tag.
#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Subcategory {
    #[serde(rename = "$attr:text")]
    pub text: Option<SubcategoryName>,
}

/// Artwork associated with a podcast or episode.
#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Image {
    #[serde(rename = "$attr:href")]
    pub href: Option<String>,
}

/// Podcast owner's contact information.
#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Owner {
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:email")]
    pub email: Option<String>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:name")]
    pub name: Option<String>,
}

/// Used for deserializing values in some of [`itunes`](crate::itunes) tags.
///
/// The only possible value in some of the tags is "Yes", which this enum is meant to reflect.
#[derive(Debug, PartialEq, Eq)]
pub enum Yes {
    Ok,
    Other(String),
}

impl<'de> Deserialize<'de> for Yes {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match s.as_str() {
            "Yes" => Ok(Self::Ok),
            _ => Ok(Self::Other(s)),
        }
    }
}

/// Apple Podcasts podcast type.
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum PodcastType {
    Episodic,
    Serial,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for PodcastType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match PodcastType::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(PodcastType::Other(s)),
        }
    }
}

/// Apple Podcasts episode type.
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum EpisodeType {
    Full,
    Trailer,
    Bonus,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for EpisodeType {
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
