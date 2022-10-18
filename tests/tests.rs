use chrono::prelude::*;
use parse_rss::*;

#[test]
fn deserialize() {
    let rss = parse_rss::from_str(
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
    <podcast:guid>917393e3-1b1e-5cef-ace4-edaa54e1f810</podcast:guid>
    <unknownNS:tag>val</unknownNS:tag>
    <podcast:person href="https://example.com/johnsmith/blog" img="http://example.com/images/johnsmith.jpg">John Smith</podcast:person>
    <podcast:person role="guest" href="https://www.imdb.com/name/nm0427852888/" img="http://example.com/images/janedoe.jpg">Jane Doe</podcast:person>
    <podcast:location geo="geo:33.51601,-86.81455" osm="R6930627">Birmingham Civil Rights Museum</podcast:location>
    <podcast:trailer pubdate="Thu, 01 Apr 2021 08:00:00 EST" url="https://example.org/trailers/teaser" length="12345678" type="audio/mpeg">Coming April 1st, 2021</podcast:trailer>
    <unknown1>val</unknown1>
    <podcast:license>cc-by-4.0</podcast:license>
    <unknown2>val</unknown2>
    <podcast:block>yes</podcast:block>
    <podcast:block id="youtube">no</podcast:block>
    <podcast:block id="amazon">no</podcast:block>
    <podcast:value type="lightning" method="keysend" suggested="0.00000015000">
        <podcast:valueRecipient
            name="Host"
            type="node"
            address="032f4ffbbafffbe51726ad3c164a3d0d37ec27bc67b29a159b0f49ae8ac21b8508"
            split="40"
        />
        <podcast:valueRecipient
            name="Producer"
            type="node"
            address="03ae9f91a0cb8ff43840e3c322c4c61f019d8c1c3cea15a25cfc425ac605e61a4a"
            split="10"
        />
    </podcast:value>
    <podcast:medium>music</podcast:medium>
    <podcast:images
        srcset="https://example.com/images/ep1/pci_avatar-massive.jpg 1500w,
                https://example.com/images/ep1/pci_avatar-middle.jpg 6o0w,
                https://example.com/images/ep1/pci_avatar-small.jpg 300w"
    />
    <podcast:liveItem status="live" start="2021-09-26T07:30:00.000-0600" end="2021-09-26T09:30:00.000-0600">
        <title>Podcasting 2.0 Live Stream</title>
        <guid>e32b4890-983b-4ce5-8b46-f2d6bc1d8819</guid>
        <enclosure url="https://example.com/pc20/livestream?format=.mp3" type="audio/mpeg" length="312" />
        <podcast:contentLink href="https://example.com/html/livestream">Listen Live!</podcast:contentLink>
    </podcast:liveItem>
    <item>
      <enclosure
       url="http://example.com/episode-1.mp3" 
       length="100200"
       type="audio/mpeg"
       unknown_attr="val"
      />
      <itunes:block>yes</itunes:block>
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
      <podcast:alternateEnclosure type="audio/mpeg" length="2490970" bitrate="160707.74">
        <podcast:source uri="https://example.com/file-0.mp3" />
        <podcast:source uri="ipfs://QmdwGqd3d2gFPGeJNLLCshdiPert45fMu84552Y4XHTy4y" />
        <podcast:source uri="https://example.com/file-0.torrent" contentType="application/x-bittorrent" />
        <podcast:source uri="http://example.onion/file-0.mp3" />
      </podcast:alternateEnclosure>

      <podcast:alternateEnclosure type="video/mp4" length="10562995" bitrate="681483.55" height="1080">
        <podcast:source uri="https://example.com/file-1080.mp4" />
      </podcast:alternateEnclosure>
      <podcast:value type="lightning" method="keysend" suggested="0.00000015000">
      </podcast:value>
      <podcast:socialInteract uri="https://podcastindex.social/web/@dave/108013847520053258" protocol="activitypub" accountId="@dave" />
    </item>
  </channel>
</rss>
            "#,
    ).unwrap();

    pretty_assertions::assert_eq!(
        rss,
        RSS {
            version: Some("2.0".to_string()),
            channel: Some(Channel {
                copyright: Some("© Example Company".to_string()),
                description: Some("<p><strong>Example HTML description</strong></p>".to_string()),
                language: Some(Language::EnglishUnitedStates),
                link: Some("https://example.com".to_string()),
                title: Some("Example Podcast".to_string()),
                content_encoded: Some(
                    "<p><strong>Example HTML description</strong></p>".to_string()
                ),
                itunes_author: Some("Jane Doe".to_string()),
                itunes_block: Some(itunes::Yes::Ok),
                itunes_complete: Some(itunes::Yes::Other("No".to_string())),
                itunes_categories: vec! {itunes::Category{
                    text: Some(itunes::CategoryName::SocietyAndCulture),
                    subcategory: Some(itunes::Subcategory{
                        text: Some(itunes::SubcategoryName::Documentary),
                    }),
                }},
                itunes_explicit: Some(Bool::Ok(false)),
                itunes_owner: Some(itunes::Owner {
                    email: Some("jane@example.com".to_string()),
                    name: Some("Jane Doe".to_string()),
                }),
                itunes_type: Some(itunes::PodcastType::Serial),
                podcast_locked: Some(podcast::Locked {
                    owner: None,
                    value: Some(Bool::Ok(false)),
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
                podcast_blocks: vec! {
                    podcast::Block{
                        id: None,
                        value: Some(Bool::Ok(true)),
                    },
                    podcast::Block{
                        id: Some(podcast::Service::YouTube),
                        value: Some(Bool::Ok(false)),
                    },
                    podcast::Block{
                        id: Some(podcast::Service::Amazon),
                        value: Some(Bool::Ok(false)),
                    },
                },
                podcast_location: Some(podcast::Location {
                    geo: Some(podcast::Geo::Ok(podcast::GeoCoordinates {
                        latitude: 33.51601,
                        longitude: -86.81455,
                        altitude: None,
                        uncertainty: None
                    })),
                    osm: Some(podcast::OSM::Ok(podcast::OSMObject {
                        type_: podcast::OSMType::Relation,
                        id: 6930627,
                        revision: None,
                    })),
                    value: Some("Birmingham Civil Rights Museum".to_string()),
                }),
                podcast_trailers: vec! {
                    podcast::Trailer{
                        pub_date: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(5*60*60).ymd(2021, 4, 1).and_hms(8, 0, 0))),
                        url: Some("https://example.org/trailers/teaser".to_string()),
                        length: Some(Integer::Ok(12345678)),
                        type_: Some(mimetype::Enclosure::MP3),
                        season: None,
                        value: Some("Coming April 1st, 2021".to_string()),
                    },
                },
                podcast_license: Some(podcast::License {
                    url: None,
                    value: Some(podcast::LicenseType::CreativeCommonsAttribution4_0International),
                }),
                podcast_guid: Some(podcast::GUID::Ok(
                    "917393e3-1b1e-5cef-ace4-edaa54e1f810".to_string()
                )),
                podcast_value: Some(podcast::Value {
                    type_: Some(podcast::ValueType::Lightning),
                    method: Some(podcast::ValueMethod::Keysend),
                    suggested: Some(Float::Ok(0.00000015)),
                    value_recipients: vec! {
                        podcast::ValueRecipient{
                            name: Some("Host".to_string()),
                            type_: Some(podcast::ValueRecipientType::Node),
                            address: Some("032f4ffbbafffbe51726ad3c164a3d0d37ec27bc67b29a159b0f49ae8ac21b8508".to_string()),
                            split: Some(Integer::Ok(40)),
                            ..Default::default()
                        },
                        podcast::ValueRecipient{
                            name: Some("Producer".to_string()),
                            type_: Some(podcast::ValueRecipientType::Node),
                            address: Some("03ae9f91a0cb8ff43840e3c322c4c61f019d8c1c3cea15a25cfc425ac605e61a4a".to_string()),
                            split: Some(Integer::Ok(10)),
                            ..Default::default()
                        },
                    },
                }),
                podcast_medium: Some(podcast::Medium::Music),
                podcast_images: Some(podcast::Images {
                    srcset: vec! {
                        podcast::Image::Ok("https://example.com/images/ep1/pci_avatar-massive.jpg".to_string(), 1500),
                        podcast::Image::Other("https://example.com/images/ep1/pci_avatar-middle.jpg 6o0w".to_string()),
                        podcast::Image::Ok("https://example.com/images/ep1/pci_avatar-small.jpg".to_string(), 300),
                    },
                }),
                items: vec! {
                Item{
                    title: Some("Example Episode".to_string()),
                    enclosure: Some(Enclosure{
                        url: Some("http://example.com/episode-1.mp3".to_string()),
                        length: Some(Integer::Ok(100200)),
                        type_: Some(mimetype::Enclosure::MP3),
                    }),
                    itunes_duration: Some(Number::Integer(1079)),
                    itunes_explicit: Some(Bool::Ok(true)),
                    pub_date: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2022, 10, 10).and_hms(6, 10, 5))),

                    podcast_chapters: Some(podcast::Chapters{
                        url: Some("https://example.com/episode-1/chapters.json".to_string()),
                        type_: Some(podcast::ChaptersType::ApplicationJSONChapters),
                    }),
                    podcast_soundbites: vec! {
                        podcast::Soundbite{
                            start_time: Some(Float::Ok(73.0)),
                            duration: Some(Float::Ok(60.0)),
                            value: None,
                        },
                        podcast::Soundbite{
                            start_time: Some(Float::Ok(1234.5)),
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
                        osm: Some(podcast::OSM::Ok(podcast::OSMObject {
                            type_: podcast::OSMType::Way,
                            id: 43678282,
                            revision: None,
                        })),
                        value: Some("Dreamworld (Queensland)".to_string()),
                    }),
                    podcast_season: Some(podcast::Season{
                        name: Some("Egyptology: The 19th Century".to_string()),
                        value: Some(Integer::Ok(1)),
                    }),
                    podcast_episode: Some(podcast::Episode{
                        display: Some("Ch.3".to_string()),
                        value: Some(Number::Integer(204)),
                    }),
                    itunes_episode: Some(Integer::Ok(204)),
                    itunes_season: Some(Integer::Other("Season 1".to_string())),
                    podcast_transcripts: vec! {
                        podcast::Transcript{
                            url: Some("https://example.com/episode1/transcript.json".to_string()),
                            type_: Some(mimetype::Transcript::JSON),
                            language: Some(Language::Spanish),
                            rel: Some(podcast::TranscriptRel::Captions),
                        },
                    },
                    itunes_block: Some(itunes::Yes::Other("yes".to_string())),
                    podcast_alternate_enclosures: vec!{
                        podcast::AlternateEnclosure{
                            type_: Some(mimetype::Enclosure::MP3),
                            length: Some(Integer::Ok(2490970)),
                            bit_rate: Some(Float::Ok(160707.74)),
                            podcast_sources: vec!{
                                podcast::Source{
                                    uri: Some("https://example.com/file-0.mp3".to_string()),
                                    type_: None,
                                },
                                podcast::Source{
                                    uri: Some("ipfs://QmdwGqd3d2gFPGeJNLLCshdiPert45fMu84552Y4XHTy4y".to_string()),
                                    type_: None,
                                },
                                podcast::Source{
                                    uri: Some("https://example.com/file-0.torrent".to_string()),
                                    type_: Some(mimetype::Enclosure::Other("application/x-bittorrent".to_string())),
                                },
                                podcast::Source{
                                    uri: Some("http://example.onion/file-0.mp3".to_string()),
                                    type_: None,
                                },
                            },
                            ..Default::default()
                        },
                        podcast::AlternateEnclosure{
                            type_: Some(mimetype::Enclosure::MP4),
                            length: Some(Integer::Ok(10562995)),
                            bit_rate: Some(Float::Ok(681483.55)),
                            height: Some(Integer::Ok(1080)),
                            podcast_sources: vec!{
                                podcast::Source{
                                    uri: Some("https://example.com/file-1080.mp4".to_string()),
                                    type_: None,
                                },
                            },
                            ..Default::default()
                        },
                    },
                    podcast_value: Some(podcast::Value {
                        type_: Some(podcast::ValueType::Lightning),
                        method: Some(podcast::ValueMethod::Keysend),
                        suggested: Some(Float::Ok(0.00000015)),
                        value_recipients:vec!{},
                    }),
                    podcast_social_interacts: vec! {
                        podcast::SocialInteract{
                            uri: Some("https://podcastindex.social/web/@dave/108013847520053258".to_string()),
                            protocol: Some(podcast::SocialProtocol::ActivityPub),
                            account_id: Some("@dave".to_string()),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                }},
                podcast_live_items: vec! {
                    podcast::LiveItem{
                        status: Some(podcast::LiveItemStatus::Live),
                        start: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(6*60*60).ymd(2021, 9, 26).and_hms(7, 30, 0))),
                        end: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(6*60*60).ymd(2021, 9, 26).and_hms(9, 30, 0))),
                        title: Some("Podcasting 2.0 Live Stream".to_string()),
                        guid: Some(GUID{
                            is_permalink: None,
                            value: Some("e32b4890-983b-4ce5-8b46-f2d6bc1d8819".to_string()),
                        }),
                        enclosure: Some(Enclosure{
                            url: Some("https://example.com/pc20/livestream?format=.mp3".to_string()),
                            length: Some(Integer::Ok(312)),
                            type_: Some(mimetype::Enclosure::MP3),
                        }),
                        content_links: vec!{
                            podcast::ContentLink{
                                href: Some("https://example.com/html/livestream".to_string()),
                                value: Some("Listen Live!".to_string()),
                            },
                        },
                        ..Default::default()
                    },
                },
                ..Default::default()
            }),
        }
    );
}
