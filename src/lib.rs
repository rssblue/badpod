mod itunes_category;
mod language;

use crate::itunes_category::{ItunesCategoryName, ItunesSubcategoryName};
use crate::language::Language;
use serde::Deserialize;
use serde_enum_str::Deserialize_enum_str;

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Feed {
    pub rss: RSS,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct RSS {
    #[serde(rename = "$attr:version")]
    pub version: Option<String>,

    pub channel: Option<Channel>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Channel {
    pub copyright: Option<String>,
    pub description: Option<String>,
    pub generator: Option<String>,
    pub language: Option<Language>,
    pub link: Option<String>,
    pub title: Option<String>,

    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:author")]
    pub itunes_author: Option<String>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:block")]
    pub itunes_block: Option<ItunesYes>,
    #[serde(
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:category",
        default
    )]
    pub itunes_categories: Vec<ItunesCategory>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:complete")]
    pub itunes_complete: Option<ItunesYes>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:explicit")]
    pub itunes_explicit: Option<bool>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:image")]
    pub itunes_image: Option<ItunesImage>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:new-feed-url")]
    pub itunes_new_feed_url: Option<String>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:owner")]
    pub itunes_owner: Option<ItunesOwner>,
    #[serde(rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:type")]
    pub itunes_type: Option<ItunesPodcastType>,
}

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_element_into_struct() {
        let feed = xml_serde::from_str::<super::Feed>(
                r#"
<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd" xmlns:podcast="https://podcastindex.org/namespace/1.0">
  <channel>
    <copyright>© Example Company</copyright>
    <description><![CDATA[<p><strong>Example HTML description</strong></p>]]></description>
    <language>en-us</language>
    <link>https://example.com</link>
    <title>Example Podcast</title>
    <itunes:author>Jane Doe</itunes:author>
    <itunes:block>Yes</itunes:block>
    <itunes:complete>No</itunes:complete>
    <itunes:category text="Society &amp; Culture">
      <itunes:category text="Documentary"></itunes:category>
    </itunes:category>
    <itunes:owner>
      <itunes:name>Jane Doe</itunes:name>
      <itunes:email>jane@example.com</itunes:email>
    </itunes:owner>
    <itunes:type>serial</itunes:type>
  </channel>
</rss>
            "#
            )
            .unwrap();

        assert_eq!(
            feed,
            Feed {
                rss: RSS {
                    version: Some("2.0".to_string()),
                    channel: Some(Channel {
                        copyright: Some("© Example Company".to_string()),
                        description: Some(
                            "<p><strong>Example HTML description</strong></p>".to_string()
                        ),
                        language: Some(Language::EnglishUnitedStates),
                        link: Some("https://example.com".to_string()),
                        title: Some("Example Podcast".to_string()),
                        itunes_author: Some("Jane Doe".to_string()),
                        itunes_block: Some(ItunesYes::Yes),
                        itunes_complete: Some(ItunesYes::Other("No".to_string())),
                        itunes_categories: vec! {ItunesCategory{
                            text: Some(ItunesCategoryName::SocietyAndCulture),
                            subcategory: Some(ItunesSubcategory{
                                text: Some(ItunesSubcategoryName::Documentary),
                            }),
                        }},
                        itunes_owner: Some(ItunesOwner {
                            email: Some("jane@example.com".to_string()),
                            name: Some("Jane Doe".to_string()),
                        }),
                        itunes_type: Some(ItunesPodcastType::Serial),
                        ..Default::default()
                    }),
                }
            }
        );
    }
}
