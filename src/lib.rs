use serde::Deserialize;

mod bool;
mod language;
mod numbers;
mod time;

pub mod itunes;
pub mod podcast;

pub use crate::bool::Bool;
pub use crate::language::Language;
pub use crate::numbers::Float;
pub use crate::time::DateTime;

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Feed {
    pub rss: RSS,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct RSS {
    #[serde(rename = "$attr:version")]
    pub version: Option<String>,

    pub channel: Option<Channel>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
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
        default,
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
        rename = "{https://podcastindex.org/namespace/1.0}podcast:locked"
    )]
    pub podcast_locked: Option<podcast::Locked>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:funding",
        default
    )]
    pub podcast_fundings: Vec<podcast::Funding>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:person",
        default
    )]
    pub podcast_persons: Vec<podcast::Person>,

    #[serde(rename = "item", default)]
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Item {
    pub description: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
    pub enclosure: Option<Enclosure>,
    pub guid: Option<GUID>,
    #[serde(default, deserialize_with = "time::option_datefmt", rename = "pubDate")]
    pub pub_date: Option<DateTime>,

    #[serde(rename = "{https://podcastindex.org/namespace/1.0}podcast:chapters")]
    pub podcast_chapters: Option<podcast::Chapters>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:soundbite",
        default
    )]
    pub podcast_soundbites: Vec<podcast::Soundbite>,
    #[serde(
        rename = "{https://podcastindex.org/namespace/1.0}podcast:person",
        default
    )]
    pub podcast_persons: Vec<podcast::Person>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct Enclosure {
    #[serde(rename = "$attr:url")]
    pub url: Option<String>,
    #[serde(rename = "$attr:length")]
    pub length: Option<usize>,
    #[serde(rename = "$attr:type")]
    pub type_: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Default)]
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
    <podcast:funding url="https://www.example.com/donations">Support the show!</podcast:funding>
    <podcast:funding url="https://www.example.com/members">Become a member!</podcast:funding>
    <podcast:person href="https://example.com/johnsmith/blog" img="http://example.com/images/johnsmith.jpg">John Smith</podcast:person>
    <podcast:person role="guest" href="https://www.imdb.com/name/nm0427852888/" img="http://example.com/images/janedoe.jpg">Jane Doe</podcast:person>
    <item>
      <enclosure
       url="http://example.com/episode-1.mp3" 
       length="100200"
       type="audio/mpeg"
      />
      <pubDate>Mon, 10 Oct 2022 06:10:05 GMT</pubDate>
      <title>Example Episode</title>
      <podcast:chapters url="https://example.com/episode-1/chapters.json" type="application/json+chapters" />
      <podcast:soundbite startTime="73.0" duration="60.0" />
      <podcast:soundbite startTime="1234.5" duration="42.25">Why the Podcast Namespace Matters</podcast:soundbite>
      <podcast:person role="guest" href="https://www.wikipedia/alicebrown" img="http://example.com/images/alicebrown.jpg">Alice Brown</podcast:person>
      <podcast:person group="Writing" role="Guest" href="https://www.wikipedia/alicebrown" img="http://example.com/images/alicebrown.jpg">Alice Brown</podcast:person>
      <podcast:person group="non-existent group" role="Non-existent role" href="https://example.com/artist/beckysmith">Becky Smith</podcast:person>
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
                        podcast_locked: Some(podcast::Locked {
                            owner: None,
                            value: Some(Bool::Bool(false)),
                        }),
                        podcast_fundings: vec! {
                            podcast::Funding{
                                url: Some("https://www.example.com/donations".to_string()),
                                value: Some("Support the show!".to_string()),
                            },
                            podcast::Funding{
                                url: Some("https://www.example.com/members".to_string()),
                                value: Some("Become a member!".to_string()),
                            },
                        },
                        podcast_persons: vec! {
                            podcast::Person{
                                href: Some("https://example.com/johnsmith/blog".to_string()),
                                img: Some("http://example.com/images/johnsmith.jpg".to_string()),
                                value: Some("John Smith".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                role: Some(podcast::PersonRole::Guest),
                                href: Some("https://www.imdb.com/name/nm0427852888/".to_string()),
                                img: Some("http://example.com/images/janedoe.jpg".to_string()),
                                value: Some("Jane Doe".to_string()),
                                ..Default::default()
                            },
                        },
                        items: vec! {
                        Item{
                        title: Some("Example Episode".to_string()),
                        enclosure: Some(Enclosure{
                            url: Some("http://example.com/episode-1.mp3".to_string()),
                            length: Some(100200),
                            type_: Some("audio/mpeg".to_string()),
                        }),
                        pub_date: Some(time::DateTime::Rfc2822(chrono::FixedOffset::east(0).ymd(2022, 10, 10).and_hms(6, 10, 5))),

                        podcast_chapters: Some(podcast::Chapters{
                            url: Some("https://example.com/episode-1/chapters.json".to_string()),
                            type_: Some(podcast::ChaptersType::ApplicationJSONChapters),
                        }),
                        podcast_soundbites: vec! {
                            podcast::Soundbite{
                                start_time: Some(Float::Float(73.0)),
                                duration: Some(Float::Float(60.0)),
                                value: None,
                            },
                            podcast::Soundbite{
                                start_time: Some(Float::Float(1234.5)),
                                duration: Some(Float::Float(42.25)),
                                value: Some("Why the Podcast Namespace Matters".to_string()),
                            },
                        },
                        podcast_persons: vec! {
                            podcast::Person{
                                role: Some(podcast::PersonRole::Guest),
                                href: Some("https://www.wikipedia/alicebrown".to_string()),
                                img: Some("http://example.com/images/alicebrown.jpg".to_string()),
                                value: Some("Alice Brown".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                group: Some(podcast::PersonGroup::Writing),
                                role: Some(podcast::PersonRole::Guest),
                                href: Some("https://www.wikipedia/alicebrown".to_string()),
                                img: Some("http://example.com/images/alicebrown.jpg".to_string()),
                                value: Some("Alice Brown".to_string()),
                            },
                            podcast::Person{
                                group: Some(podcast::PersonGroup::Other("non-existent group".to_string())),
                                role: Some(podcast::PersonRole::Other("Non-existent role".to_string())),
                                href: Some("https://example.com/artist/beckysmith".to_string()),
                                value: Some("Becky Smith".to_string()),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                        }},
                        ..Default::default()
                    }),
                }
            }
        );
    }
}
