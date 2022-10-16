use super::*;
use chrono::prelude::*;

#[test]
fn deserialize_element_into_struct() {
    let feed = xml_serde::from_str::<super::Feed>(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd" xmlns:podcast="https://podcastindex.org/namespace/1.0" xmlns:unknownNS="https://example.com">
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
    <unknownNS:tag>val</unknownNS:tag>
    <podcast:person href="https://example.com/johnsmith/blog" img="http://example.com/images/johnsmith.jpg">John Smith</podcast:person>
    <podcast:person role="guest" href="https://www.imdb.com/name/nm0427852888/" img="http://example.com/images/janedoe.jpg">Jane Doe</podcast:person>
    <podcast:location geo="geo:33.51601,-86.81455" osm="R6930627">Birmingham Civil Rights Museum</podcast:location>
    <podcast:trailer pubdate="Thu, 01 Apr 2021 08:00:00 EST" url="https://example.org/trailers/teaser" length="12345678" type="audio/mpeg">Coming April 1st, 2021</podcast:trailer>
    <unknown1>val</unknown1>
    <podcast:license>cc-by-4.0</podcast:license>
    <unknown2>val</unknown2>
    <item>
      <enclosure
       url="http://example.com/episode-1.mp3" 
       length="100200"
       type="audio/mpeg"
       unknown_attr="val"
      />
      <itunes:explicit>true</itunes:explicit>
      <itunes:duration>1079</itunes:duration>
      <pubDate>Mon, 10 Oct 2022 06:10:05 GMT</pubDate>
      <title>Example Episode</title>
      <podcast:chapters url="https://example.com/episode-1/chapters.json" type="application/json+chapters" />
      <podcast:soundbite startTime="73.0" duration="60.0" />
      <podcast:soundbite startTime="1234.5" duration="-42.25">Why the Podcast Namespace Matters</podcast:soundbite>
      <podcast:person role="guest" href="https://www.wikipedia/alicebrown" img="http://example.com/images/alicebrown.jpg">Alice Brown</podcast:person>
      <podcast:person group="Writing" role="Guest" href="https://www.wikipedia/alicebrown" img="http://example.com/images/alicebrown.jpg">Alice Brown</podcast:person>
      <podcast:person group="non-existent group" role="Non-existent role" href="https://example.com/artist/beckysmith">Becky Smith</podcast:person>
      <podcast:location geo="GEO:-27.86159,153.3169" osm="W43678282">Dreamworld (Queensland)</podcast:location>
      <podcast:episode display="Ch.3">204</podcast:episode>
      <itunes:episode>204</itunes:episode>
      <itunes:season>Season 1</itunes:season>
      <podcast:season name="Egyptology: The 19th Century">1</podcast:season>
      <podcast:transcript url="https://example.com/episode1/transcript.json" type="application/json" language="es" rel="captions" />
      <itunes:episodeType>full</itunes:episodeType>
    </item>
  </channel>
</rss>
            "#
            )
            .unwrap();

    pretty_assertions::assert_eq!(
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
                    podcast_location: Some(podcast::Location {
                        geo: Some(podcast::Geo::Geo(podcast::GeoCoordinates {
                            latitude: 33.51601,
                            longitude: -86.81455,
                            altitude: None,
                            uncertainty: None
                        })),
                        osm: Some(podcast::OSM::OSM(podcast::OSMObject {
                            type_: podcast::OSMType::Relation,
                            id: 6930627,
                            revision: None,
                        })),
                        value: Some("Birmingham Civil Rights Museum".to_string()),
                    }),
                    podcast_trailers: vec! {
                        podcast::Trailer{
                            pub_date: Some(time::DateTime::Rfc2822(chrono::FixedOffset::west(5*60*60).ymd(2021, 4, 1).and_hms(8, 0, 0))),
                            url: Some("https://example.org/trailers/teaser".to_string()),
                            length: Some(Integer::Integer(12345678)),
                            type_: Some(mimetype::Enclosure::MP3),
                            season: None,
                            value: Some("Coming April 1st, 2021".to_string()),
                        },
                    },
                    podcast_license: Some(podcast::License {
                        url: None,
                        value: Some(
                            podcast::LicenseType::CreativeCommonsAttribution4_0International
                        ),
                    }),
                    items: vec! {
                    Item{
                        title: Some("Example Episode".to_string()),
                        enclosure: Some(Enclosure{
                            url: Some("http://example.com/episode-1.mp3".to_string()),
                            length: Some(100200),
                            type_: Some(mimetype::Enclosure::MP3),
                        }),
                        itunes_duration: Some(Number::Integer(1079)),
                        itunes_explicit: Some(Bool::Bool(true)),
                        pub_date: Some(time::DateTime::Rfc2822(chrono::FixedOffset::west(5).ymd(2022, 10, 10).and_hms(6, 10, 0))),

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
                                duration: Some(Float::Other("-42.25".to_string())),
                                value: Some("Why the Podcast Namespace Matters".to_string()),
                            },
                        },
                        itunes_type: Some(itunes::EpisodeType::Full),
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
                        podcast_location: Some(podcast::Location {
                            geo: Some(podcast::Geo::Other("GEO:-27.86159,153.3169".to_string())),
                            osm: Some(podcast::OSM::OSM(podcast::OSMObject {
                                type_: podcast::OSMType::Way,
                                id: 43678282,
                                revision: None,
                            })),
                            value: Some("Dreamworld (Queensland)".to_string()),
                        }),
                        podcast_season: Some(podcast::Season{
                            name: Some("Egyptology: The 19th Century".to_string()),
                            value: Some(Integer::Integer(1)),
                        }),
                        podcast_episode: Some(podcast::Episode{
                            display: Some("Ch.3".to_string()),
                            value: Some(Number::Integer(204)),
                        }),
                        itunes_episode: Some(Integer::Integer(204)),
                        itunes_season: Some(Integer::Other("Season 1".to_string())),
                        podcast_transcripts: vec! {
                            podcast::Transcript{
                                url: Some("https://example.com/episode1/transcript.json".to_string()),
                                type_: Some(mimetype::Transcript::JSON),
                                language: Some(Language::Spanish),
                                rel: Some(podcast::TranscriptRel::Captions),
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
