use crate::utils;
use strum_macros::EnumIter;

mod category;
pub use category::{CategoryName, SubcategoryName};

/// Apple Podcasts podcast category.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Category {
    pub text: Option<CategoryName>,
    pub subcategory: Vec<Subcategory>,
}

/// Apple Podcasts podcast subcategory.
///
/// Just because a subcategory is deserialized does not mean that it is compatible with the
/// category in the parent tag.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Subcategory {
    pub text: Option<SubcategoryName>,
}

/// Artwork associated with a podcast or episode.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Image {
    pub href: Option<String>,
}

/// Podcast owner's contact information.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Owner {
    pub email: Vec<String>,
    pub name: Vec<String>,
}

/// Used for deserializing values in some of [`itunes`](crate::itunes) tags.
///
/// The only possible value in some of the tags is "Yes", which this enum is meant to reflect.
#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum Yes {
    Ok,
    Other(String),
}

impl Yes {
    pub fn parse(s: &str) -> Self {
        match s {
            "Yes" => Yes::Ok,
            _ => Yes::Other(s.to_string()),
        }
    }
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

impl PodcastType {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(variant) => variant,
            Err(_) => Self::Other(s.to_string()),
        }
    }
}

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

impl EpisodeType {
    pub fn parse(s: &str) -> Self {
        match s.parse() {
            Ok(variant) => variant,
            Err(_) => Self::Other(s.to_string()),
        }
    }
}
