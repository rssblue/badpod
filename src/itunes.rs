use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;

mod category;
pub use category::{ItunesCategoryName, ItunesSubcategoryName};

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct ItunesCategory {
    #[serde(rename = "$attr:text")]
    pub text: Option<ItunesCategoryName>,

    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:category")]
    pub subcategory: Option<ItunesSubcategory>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct ItunesSubcategory {
    #[serde(rename = "$attr:text")]
    pub text: Option<ItunesSubcategoryName>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct ItunesImage {
    #[serde(rename = "$attr:href")]
    pub href: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct ItunesOwner {
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:email")]
    pub email: Option<String>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:name")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize_enum_str, PartialEq, Eq)]
pub enum ItunesYes {
    Yes,
    #[serde(other)]
    Other(String),
}

#[derive(Debug, Deserialize_enum_str, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ItunesPodcastType {
    Episodic,
    Serial,
    #[serde(other)]
    Other(String),
}
