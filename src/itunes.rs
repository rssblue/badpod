use crate::utils;
use strum_macros::EnumIter;

mod category;
pub use category::{CategoryName, SubcategoryName};

/// Apple Podcasts podcast category.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Category {
    // #[serde(rename = "$attr:text")]
    pub text: Option<CategoryName>,

    // #[serde(
    //     default,
    //     rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:category"
    // )]
    pub subcategory: Vec<Subcategory>,
}

/// Apple Podcasts podcast subcategory.
///
/// Just because a subcategory is deserialized does not mean that it is compatible with the
/// category in the parent tag.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Subcategory {
    // #[serde(rename = "$attr:text")]
    pub text: Option<SubcategoryName>,
}

/// Artwork associated with a podcast or episode.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Image {
    // #[serde(rename = "$attr:href")]
    pub href: Option<String>,
}

/// Podcast owner's contact information.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Owner {
    // #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:email")]
    pub email: Option<String>,
    // #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:name")]
    pub name: Option<String>,
}

/// Used for deserializing values in some of [`itunes`](crate::itunes) tags.
///
/// The only possible value in some of the tags is "Yes", which this enum is meant to reflect.
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Yes {
    Ok,
    Other(String),
}

impl std::str::FromStr for Yes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for Yes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => {
                write!(f, "Yes")
            }
        }
    }
}

// impl<'de> Deserialize<'de> for Yes {
//     fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         utils::deserialize_using_from_str(d)
//     }
// }

/// Apple Podcasts podcast type.
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum PodcastType {
    Episodic,
    Serial,

    Other(String),
}

impl std::str::FromStr for PodcastType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for PodcastType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => {
                let s = format!("{:?}", self);
                write!(f, "{}", s.to_lowercase())
            }
        }
    }
}

// impl<'de> Deserialize<'de> for PodcastType {
//     fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         utils::deserialize_using_from_str(d)
//     }
// }

/// Apple Podcasts episode type.
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum EpisodeType {
    Full,
    Trailer,
    Bonus,

    Other(String),
}

impl std::str::FromStr for EpisodeType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match utils::from_str_exact(s) {
            Some(variant) => Ok(variant),
            None => Ok(Self::Other(s.to_string())),
        }
    }
}

impl std::fmt::Display for EpisodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{s}"),
            _ => {
                let s = format!("{:?}", self);
                write!(f, "{}", s.to_lowercase())
            }
        }
    }
}

// impl<'de> Deserialize<'de> for EpisodeType {
//     fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
//         utils::deserialize_using_from_str(d)
//     }
// }
