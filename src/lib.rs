use serde::Deserialize;

mod bool;
mod language;
mod time;

pub mod itunes;
pub mod podcast;

pub use crate::bool::Bool;
pub use crate::language::Language;
pub use crate::time::DateTime;

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
        rename = "{http://www.itunes.com/dtds/podcast-1.0.dtd}itunes:explicit",
        deserialize_with = "bool::option_bool_tf"
    )]
    pub itunes_explicit: Option<Bool>,
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
        rename = "{https://podcastindex.org/namespace/1.0}podcast:locked",
        deserialize_with = "bool::option_bool_yn"
    )]
    pub podcast_locked: Option<Bool>,

    #[serde(rename = "item", default)]
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Item {
    pub description: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
    pub enclosure: Option<Enclosure>,
    pub guid: Option<GUID>,
    // TODO: fix.
    #[serde(default, deserialize_with = "time::option_datefmt", rename = "pubDate")]
    pub pub_date: Option<DateTime>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Enclosure {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$attr:length")]
    pub length: Option<usize>,
    #[serde(rename = "$attr:type")]
    pub type_: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct GUID {
    #[serde(
        rename = "$attr:isPermaLink",
        deserialize_with = "bool::option_bool_tf"
    )]
    pub is_permalink: Option<Bool>,
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

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
    <content:encoded>&lt;p&gt;&lt;strong&gt;Example HTML description&lt;/strong&gt;&lt;/p&gt;</content:encoded>
    <itunes:author>Jane Doe</itunes:author>
    <itunes:block>Yes</itunes:block>
    <itunes:complete>No</itunes:complete>
    <itunes:category text="Society &amp; Culture">
      <itunes:category text="Documentary"></itunes:category>
    </itunes:category>
    <itunes:explicit>false</itunes:explicit>
    <itunes:owner>
      <itunes:name>Jane Doe</itunes:name>
      <itunes:email>jane@example.com</itunes:email>
    </itunes:owner>
    <itunes:type>serial</itunes:type>
    <podcast:locked>no</podcast:locked>
    <item>
      <enclosure
       url="http://example.com/episode-1.mp3" 
       length="100200"
       type="audio/mpeg"
      />
      <pubDate>Mon, 10 Oct 2022 06:10:05 GMT</pubDate>
      <title>Example Episode</title>
    </item>
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
                        content_encoded: Some(
                            "<p><strong>Example HTML description</strong></p>".to_string()
                        ),
                        itunes_author: Some("Jane Doe".to_string()),
                        itunes_block: Some(itunes::Yes::Yes),
                        itunes_complete: Some(itunes::Yes::Other("No".to_string())),
                        itunes_categories: vec! {itunes::Category{
                            text: Some(itunes::CategoryName::SocietyAndCulture),
                            subcategory: Some(itunes::Subcategory{
                                text: Some(itunes::SubcategoryName::Documentary),
                            }),
                        }},
                        itunes_explicit: Some(Bool::Bool(false)),
                        itunes_owner: Some(itunes::Owner {
                            email: Some("jane@example.com".to_string()),
                            name: Some("Jane Doe".to_string()),
                        }),
                        itunes_type: Some(itunes::PodcastType::Serial),
                        podcast_locked: Some(Bool::Bool(false)),
                        items: vec! {Item{
                            title: Some("Example Episode".to_string()),
                            enclosure: Some(Enclosure{
                                url: Some("http://example.com/episode-1.mp3".to_string()),
                                length: Some(100200),
                                type_: Some("audio/mpeg".to_string()),
                            }),
                            pub_date: Some(time::DateTime::Rfc2822(chrono::FixedOffset::east(0).ymd(2022, 10, 10).and_hms(6, 10, 5))),
                            ..Default::default()
                        }},
                        ..Default::default()
                    }),
                }
            }
        );
    }
}
