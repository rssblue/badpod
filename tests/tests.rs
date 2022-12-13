use chrono::prelude::*;
use badpod::*;

#[test]
fn deserialize() {
    let conditions : &[(&str, Result<Rss, badpod::Error>)] = &[
        (
            include_str!("data/empty_rss.xml"),
            Ok(Rss{
                ..Default::default()
            })
        ),
        (
            include_str!("data/empty_xml.xml"),
            Err(Error::NoRoot)
        ),
        (
            include_str!("data/empty_file.xml"),
            Err(Error::NoRoot)
        ),
        (
            include_str!("data/root_not_rss.xml"),
            Err(Error::RootNotRss)
        ),
        (
            include_str!("data/strange_feed.xml"),
            Ok(Rss{
                channel: vec![Channel{
                    itunes_category: vec![itunes::Category{
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            })
        ),
        (
            include_str!("data/simple_feed.xml"),
            Ok(Rss{
                version: Some("2.0".to_string()),
                channel: vec![Channel{
                    description: vec!["My description".to_string()],
                    title: vec!["My title".to_string()],
                    ..Default::default()
                }],
            })
        ),
        (
            include_str!("data/test_feed.xml"),
            Ok(Rss {
                version: Some("2.0".to_string()),
                channel: vec![Channel {
                    copyright: vec!["© Example Company".to_string()],
                    description: vec!["<p><strong>Example HTML description</strong></p>".to_string()],
                    language: vec![Language::Lithuanian],
                    link: vec![Url::Ok(url::Url::parse("https://example.com").unwrap())],
                    title: vec!["Example Podcast".to_string()],
                    content_encoded: vec!["<p><strong>Example HTML description</strong></p>".to_string()
                    ],
                    itunes_author: vec!["Jane Doe".to_string()],
                    itunes_block: vec![itunes::Yes::Ok],
                    itunes_complete: vec![itunes::Yes::Other(("No".to_string(), "should be \"Yes\"".to_string()))],
                    itunes_category: vec! {
                        itunes::Category{
                            text: Some(itunes::CategoryName::Music),
                            subcategory: vec![itunes::Subcategory{
                                text: Some(itunes::SubcategoryName::Other(
                                              (
                                              "Entertainment News".to_string(),
                                              "subcategory \"Entertainment News\" is not allowed for category \"Music\"; valid subcategories: \"Music Commentary\", \"Music History\", \"Music Interviews\"".to_string(),
                                              ),
                                              )),
                            }],
                        },
                        itunes::Category{
                            text: Some(itunes::CategoryName::Technology),
                            subcategory: vec![],
                        },
                    },
                    itunes_explicit: vec![Bool::Ok(false)],
                    itunes_owner: vec![itunes::Owner {
                        email: vec!["jane@example.com".to_string()],
                        name: vec!["Jane Doe".to_string()],
                    }],
                    itunes_type: vec![itunes::PodcastType::Serial],
                    podcast_locked: vec![podcast::Locked {
                        owner: None,
                        value: Some(Bool::Ok(false)),
                    }],
                    podcast_funding: vec![
                        podcast::Funding{
                            url: Some(Url::Ok(url::Url::parse("https://www.example.com/donations").unwrap())),
                            value: Some("Support the show!".to_string()),
                        },
                        podcast::Funding{
                            url: Some(Url::Ok(url::Url::parse("https://www.example.com/members").unwrap())),
                            value: Some("Become a member!".to_string()),
                        },
                    ],
                    podcast_person: vec! {
                        podcast::Person{
                            href: Some(Url::Ok(url::Url::parse("https://example.com/johnsmith/blog").unwrap())),
                            img: Some(Url::Other(("http://example.com/images/johnsmith.jpg".to_string(), "protocol must be `https`".to_string()))),
                            value: Some("John Smith".to_string()),
                            ..Default::default()
                        },
                        podcast::Person{
                            role: Some(podcast::PersonRole::Guest),
                            href: Some(Url::Ok(url::Url::parse("https://www.imdb.com/name/nm0427852888/").unwrap())),
                            img: Some(Url::Other(("http://example.com/images/janedoe.jpg".to_string(), "protocol must be `https`".to_string()))),
                            value: Some("Jane Doe".to_string()),
                            ..Default::default()
                        },
                    },
                    podcast_block: vec! {
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
                    podcast_location: vec![podcast::Location {
                        geo: Some(podcast::Geo::Ok{
                            latitude: 33.51601,
                            longitude: -86.81455,
                            altitude: None,
                            uncertainty: None
                        }),
                        osm: Some(podcast::Osm::Ok{
                            type_: podcast::OsmType::Relation,
                            id: 6930627,
                            revision: None,
                        }),
                        value: Some("Birmingham Civil Rights Museum".to_string()),
                    }],
                    podcast_trailer: vec! {
                        podcast::Trailer{
                            pub_date: Some(badpod::DateTime::Ok(chrono::FixedOffset::west(5*60*60).ymd(2021, 4, 1).and_hms(8, 0, 0))),
                            url: Some(Url::Ok(url::Url::parse("https://example.org/trailers/teaser").unwrap())),
                            length: Some(Integer::Ok(12345678)),
                            type_: Some(MimeEnclosure::AudioMp3),
                            season: None,
                            value: Some("Coming April 1st, 2021".to_string()),
                        },
                    },
                    podcast_license: vec![podcast::License {
                        url: None,
                        value: Some(podcast::LicenseType::CreativeCommonsAttribution4_0International),
                    }],
                    podcast_guid: vec![podcast::Guid::Ok(
                                      "917393e3-1b1e-5cef-ace4-edaa54e1f810".to_string()
                                      )
                    ],
                    podcast_value: vec![podcast::Value {
                        type_: Some(podcast::ValueType::Lightning),
                        method: Some(podcast::ValueMethod::Keysend),
                        suggested: Some(Float::Ok(0.00000015)),
                        value_recipient: vec! {
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
                    }],
                    podcast_medium: vec![podcast::Medium::Music],
                    podcast_images: vec![podcast::Images {
                        srcset: podcast::ImageSrcSet::Other(("https://example.com/images/ep1/pci_avatar-massive.jpg 1500w,       https://example.com/images/ep1/pci_avatar-middle.jpg 600w,       https://example.com/images/ep1/pci_avatar-small.jpg -300w".to_string(), "invalid image at index 2: invalid width (should be positive)".to_string())),
                    }],
                    item: vec! {
                        Item{
                            title: vec!["Example Episode".to_string()],
                            enclosure: vec![Enclosure{
                                url: Some(Url::Ok(url::Url::parse("http://example.com/episode-1.mp3").unwrap())),
                                length: Some(Integer::Ok(100200)),
                                type_: Some(MimeEnclosure::AudioMp3),
                            }],
                            itunes_duration: vec![
                                Number::Integer(1079),
                            ],
                            itunes_explicit: vec![Bool::Ok(true)],
                            pub_date: vec![badpod::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2022, 10, 10).and_hms(6, 10, 5))],

                            podcast_chapters: vec![podcast::Chapters{
                                url: Some(Url::Ok(url::Url::parse("https://example.com/episode-1/chapters.json").unwrap())),
                                type_: Some(MimeChapters::ApplicationJsonChapters),
                            }],
                            podcast_soundbite: vec! {
                                podcast::Soundbite{
                                    start_time: Some(Float::Ok(73.0)),
                                    duration: Some(Float::Ok(60.0)),
                                    value: None,
                                },
                                podcast::Soundbite{
                                    start_time: Some(Float::Ok(1234.5)),
                                    duration: Some(Float::Other(("-42.25".to_string(), "should be positive".to_string()))),
                                    value: Some("Why the Podcast Namespace Matters".to_string()),
                                },
                            },
                            itunes_type: vec![itunes::EpisodeType::Full],
                            podcast_person: vec! {
                                podcast::Person{
                                    role: Some(podcast::PersonRole::Guest),
                                    href: Some(Url::Ok(url::Url::parse("https://www.wikipedia/alicebrown").unwrap())),
                                    img: Some(Url::Other(("http://example.com/images/alicebrown.jpg".to_string(), "protocol must be `https`".to_string()))),
                                    value: Some("Alice Brown".to_string()),
                                    ..Default::default()
                                },
                                podcast::Person{
                                    group: Some(podcast::PersonGroup::Writing),
                                    role: Some(podcast::PersonRole::Guest),
                                    href: Some(Url::Ok(url::Url::parse("https://www.wikipedia/alicebrown").unwrap())),
                                    img: Some(Url::Other(("http://example.com/images/alicebrown.jpg".to_string(), "protocol must be `https`".to_string()))),
                                    value: Some("Alice Brown".to_string()),
                                },
                                podcast::Person{
                                    group: Some(podcast::PersonGroup::Other(("non-existent group".to_string(), "should be one of the groups defined at <https://podcasttaxonomy.com>".to_string()))),
                                    role: Some(podcast::PersonRole::Other(("Non-existent role".to_string(), "should be one of the roles defined at <https://podcasttaxonomy.com>".to_string()))),
                                    href: Some(Url::Ok(url::Url::parse("https://example.com/artist/beckysmith").unwrap())),
                                    value: Some("Becky Smith".to_string()),
                                    ..Default::default()
                                },
                            },
                            podcast_location: vec![podcast::Location {
                                geo: Some(podcast::Geo::Other(("GEO:-27.86159,153.3169".to_string(), "should start with \"geo:\"".to_string()))),
                                osm: Some(podcast::Osm::Ok{
                                    type_: podcast::OsmType::Way,
                                    id: 43678282,
                                    revision: None,
                                }),
                                value: Some("Dreamworld (Queensland)".to_string()),
                            }],
                            podcast_season: vec![podcast::Season{
                                name: Some("Egyptology: The 19th Century".to_string()),
                                value: Some(Integer::Ok(1)),
                            }],
                            podcast_episode: vec![podcast::Episode{
                                display: Some("Ch.3".to_string()),
                                value: Some(Number::Integer(204)),
                            }],
                            itunes_episode: vec![Integer::Ok(204)],
                            itunes_season: vec![
                                Integer::Other(("0".to_string(), "should be positive".to_string())),
                            ],
                            podcast_transcript: vec! {
                                podcast::Transcript{
                                    url: Some(Url::Ok(url::Url::parse("https://example.com/episode1/transcript.json").unwrap())),
                                    type_: Some(MimeTranscript::ApplicationJson),
                                    language: Some(Language::Spanish(LanguageSpanish::Default)),
                                    rel: Some(podcast::TranscriptRel::Captions),
                                },
                            },
                            itunes_block: vec![itunes::Yes::Other(("yes".to_string(), "should be \"Yes\"".to_string()))],
                            podcast_alternate_enclosure: vec!{
                                podcast::AlternateEnclosure{
                                    type_: Some(MimeEnclosure::AudioMp3),
                                    length: Some(Integer::Ok(2490970)),
                                    bit_rate: Some(Float::Ok(160707.74)),
                                    podcast_source: vec!{
                                        podcast::Source{
                                            uri: Some(Url::Ok(url::Url::parse("https://example.com/file-0.mp3").unwrap())),
                                            type_: None,
                                        },
                                        podcast::Source{
                                            uri: Some(Url::Ok(url::Url::parse("ipfs://QmdwGqd3d2gFPGeJNLLCshdiPert45fMu84552Y4XHTy4y").unwrap())),
                                            type_: None,
                                        },
                                        podcast::Source{
                                            uri: Some(Url::Ok(url::Url::parse("https://example.com/file-0.torrent").unwrap())),
                                            type_: Some(MimeEnclosure::Other(("application/x-bittorrent".to_string(), "unrecognized mime type".to_string()))),
                                        },
                                        podcast::Source{
                                            uri: Some(Url::Other(("http://example.onion/file-0.mp3".to_string(), "protocol must not be `http`".to_string()))),
                                            type_: None,
                                        },
                                    },
                                    ..Default::default()
                                },
                                podcast::AlternateEnclosure{
                                    type_: Some(MimeEnclosure::VideoMp4),
                                    length: Some(Integer::Ok(10562995)),
                                    bit_rate: Some(Float::Ok(681483.55)),
                                    height: Some(Integer::Ok(1080)),
                                    podcast_source: vec!{
                                        podcast::Source{
                                            uri: Some(Url::Ok(url::Url::parse("https://example.com/file-1080.mp4").unwrap())),
                                            type_: None,
                                        },
                                    },
                                    ..Default::default()
                                },
                            },
                            podcast_value: vec![podcast::Value {
                                type_: Some(podcast::ValueType::Lightning),
                                method: Some(podcast::ValueMethod::Keysend),
                                suggested: Some(Float::Ok(0.00000015)),
                                value_recipient: vec!{},
                            }],
                            podcast_social_interact: vec! {
                                podcast::SocialInteract{
                                    uri: Some(Url::Ok(url::Url::parse("https://podcastindex.social/web/@dave/108013847520053258").unwrap())),
                                    protocol: Some(podcast::SocialProtocol::ActivityPub),
                                    account_id: Some("@dave".to_string()),
                                    ..Default::default()
                                },
                            },
                            podcast_txt: vec![
                                podcast::Txt{
                                    purpose: None,
                                    value: Some("naj3eEZaWVVY9a38uhX8FekACyhtqP4JN".to_string()),
                                },
                                podcast::Txt{
                                    purpose: Some(podcast::TxtPurpose::Verify),
                                    value: Some("S6lpp-7ZCn8-dZfGc-OoyaG".to_string()),
                                },
                                podcast::Txt{
                                    purpose: Some(podcast::TxtPurpose::Other(("release".to_string(), "should be \"verify\"".to_string()))),
                                    value: Some("2022-10-26T04:45:30.742Z".to_string()),
                                },
                            ],
                            ..Default::default()
                        },
                        Item{
                            title: vec!["Episode minus 1".to_string()],
                            ..Default::default()
                        },
                        Item{
                            title: vec!["Episode minus 2".to_string()],
                            ..Default::default()
                        },
                    },
                    podcast_live_item: vec! {
                        podcast::LiveItem{
                            status: Some(podcast::LiveItemStatus::Live),
                            start: Some(badpod::DateTime::Ok(chrono::FixedOffset::west(6*60*60).ymd(2021, 9, 26).and_hms(7, 30, 0))),
                            end: Some(badpod::DateTime::Ok(chrono::FixedOffset::west(6*60*60).ymd(2021, 9, 26).and_hms(9, 30, 0))),
                            title: vec!["Podcasting 2.0 Live Stream".to_string()],
                            guid: vec![Guid{
                                is_permalink: None,
                                value: Some(GuidValue::Other(("e32b4890-983b-4ce5-8b46-f2d6bc1d8819".to_string(), "should be a URL when `isPermalink` is not set".to_string()))),
                            }],
                            enclosure: vec![Enclosure{
                                url: Some(Url::Ok(url::Url::parse("https://example.com/pc20/livestream?format=.mp3").unwrap())),
                                length: Some(Integer::Ok(312)),
                                type_: Some(MimeEnclosure::AudioMp3),
                            }],
                            podcast_content_link: vec!{
                                podcast::ContentLink{
                                    href: Some(Url::Ok(url::Url::parse("https://example.com/html/livestream").unwrap())),
                                    value: Some("Listen Live!".to_string()),
                                },
                            },
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                }],
            })
    ),
    (
        include_str!("data/podcast_namespace_example.xml"),
        Ok(Rss{
            version: Some("2.0".to_string()),
            channel: vec![Channel{
                title: vec!["Podcasting 2.0 Namespace Example".to_string()],
                description: vec!["This is a fake show that exists only as an example of the \"podcast\" namespace tag usage.".to_string()],
                docs: vec![Url::Ok(url::Url::parse("http://blogs.law.harvard.edu/tech/rss").unwrap())],
                link: vec![Url::Ok(url::Url::parse("http://example.com/podcast").unwrap())],
                language: vec![Language::English(LanguageEnglish::UnitedStates)],
                generator: vec!["Freedom Controller".to_string()],
                managing_editor: vec!["john@example.com (John Doe)".to_string()],
                web_master: vec!["support@example.com (Tech Support)".to_string()],
                pub_date: vec![badpod::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2020, 10, 9).and_hms(4, 30, 38))],
                last_build_date: vec![badpod::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2020, 10, 9).and_hms(4, 30, 38))],

                podcast_guid: vec![podcast::Guid::Other(("y0ur-gu1d-g035-h3r3".to_string(), r#"should be a [UUIDv5](https://tools.rssblue.com/podcast-guid)"#.to_string()))],
                podcast_license: vec![podcast::License{
                    url: Some(Url::Ok(url::Url::parse("https://example.org/mypodcastlicense/full.pdf").unwrap())),
                    value: Some(podcast::LicenseType::Other(("my-podcast-license-v1".to_string(), "unrecognized license type".to_string()))),
                }],
                podcast_locked: vec![podcast::Locked{
                    owner: Some("podcastowner@example.com".to_string()),
                    value: Some(Bool::Ok(true)),
                }],
                podcast_block: vec! {
                    podcast::Block{
                        id: None,
                        value: Some(Bool::Ok(true)),
                    },
                    podcast::Block{
                        id: Some(podcast::Service::Google),
                        value: Some(Bool::Ok(false)),
                    },
                    podcast::Block{
                        id: Some(podcast::Service::Amazon),
                        value: Some(Bool::Ok(false)),
                    },
                },
                podcast_funding: vec! {
                    podcast::Funding{
                        url: Some(Url::Ok(url::Url::parse("https://example.com/donate").unwrap())),
                        value: Some("Support the show!".to_string()),
                    },
                },
                podcast_location: vec![podcast::Location{
                    geo: Some(podcast::Geo::Ok{
                        latitude: 30.2672,
                        longitude: 97.7431,
                        altitude: None,
                        uncertainty: None,
                    }),
                    osm: Some(podcast::Osm::Ok{
                        type_: podcast::OsmType::Relation,
                        id: 113314,
                        revision: None,
                    }),
                    value: Some("Austin, TX".to_string()),
                }],
                podcast_medium: vec![podcast::Medium::Podcast],
                podcast_value: vec![podcast::Value { 
                    type_: Some(podcast::ValueType::Lightning),
                    method: Some(podcast::ValueMethod::Keysend),
                    suggested: Some(Float::Ok(0.00000005)),
                    value_recipient: vec!{
                        podcast::ValueRecipient{
                            name: Some("podcaster".to_string()),
                            type_: Some(podcast::ValueRecipientType::Node),
                            address: Some("036557ea56b3b86f08be31bcd2557cae8021b0e3a9413f0c0e52625c6696972e57".to_string()),
                            split: Some(Integer::Ok(99)),
                            ..Default::default()
                        },
                        podcast::ValueRecipient{
                            name: Some("hosting company".to_string()),
                            type_: Some(podcast::ValueRecipientType::Node),
                            address: Some("036557ea56b3b86f08be31bcd2557cae8021b0e3a9413f0c0e52625c6696972e57".to_string()),
                            split: Some(Integer::Ok(1)),
                            ..Default::default()
                        },
                    },
                }],
                podcast_trailer: vec!{
                    podcast::Trailer{
                        url: Some(Url::Ok(url::Url::parse("https://example.org/trailers/teaser").unwrap())),
                        pub_date: Some(badpod::DateTime::Ok(chrono::FixedOffset::west(5*60*60).ymd(2021, 4, 1).and_hms(8, 0, 0))),
                        length: Some(Integer::Ok(12345678)),
                        type_: Some(MimeEnclosure::Other(("audio/mp3".to_string(), "unrecognized mime type".to_string()))),
                        value: Some("Coming April 1st, 2021".to_string()),
                        season: None,
                    },
                },
                podcast_live_item: vec!{
                    podcast::LiveItem{
                        status: Some(podcast::LiveItemStatus::Live),
                        start: Some(badpod::DateTime::Ok(chrono::FixedOffset::west(6*60*60).ymd(2021, 9, 26).and_hms(7, 30, 0))),
                        end: Some(badpod::DateTime::Ok(chrono::FixedOffset::west(6*60*60).ymd(2021, 9, 26).and_hms(9, 30, 0))),
                        title: vec!["Podcasting 2.0 Live Show".to_string()],
                        description: vec!["A look into the future of podcasting and how we get to Podcasting 2.0!".to_string()],
                        link: vec![Url::Ok(url::Url::parse("https://example.com/podcast/live").unwrap())],
                        guid: vec![Guid{
                            is_permalink: Some(Bool::Ok(true)),
                            value: Some(GuidValue::Url(url::Url::parse("https://example.com/live").unwrap())),
                        }],
                        podcast_alternate_enclosure: vec!{
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioMp3),
                                length: Some(Integer::Ok(312)),
                                default: Some(Bool::Ok(true)),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/pc20/livestream").unwrap())),
                                        type_: None,
                                    },
                                },
                                ..Default::default()
                            },
                        },
                        enclosure: vec![Enclosure {
                            url: Some(Url::Ok(url::Url::parse("https://example.com/pc20/livestream?format=.mp3").unwrap())),
                            type_: Some(MimeEnclosure::AudioMp3),
                            length: Some(Integer::Ok(312)),
                        }],
                        podcast_content_link: vec!{
                            podcast::ContentLink{
                                href: Some(Url::Ok(url::Url::parse("https://youtube.com/pc20/livestream").unwrap())),
                                value: Some("YouTube!".to_string()),
                            },
                            podcast::ContentLink{
                                href: Some(Url::Ok(url::Url::parse("https://twitch.com/pc20/livestream").unwrap())),
                                value: Some("Twitch!".to_string()),
                            },
                        },
                        ..Default::default()
                    },
                },

                itunes_author: vec!["John Doe".to_string()],
                itunes_explicit: vec![Bool::Other(("no".to_string(), "should be \"true\" or \"false\"".to_string()))],
                itunes_type: vec![itunes::PodcastType::Episodic],
                itunes_category: vec!{
                    itunes::Category{
                        text: Some(itunes::CategoryName::News),
                        subcategory: vec![],
                    },
                    itunes::Category{
                        text: Some(itunes::CategoryName::Technology),
                        subcategory: vec![],
                    },
                },
                itunes_owner: vec![itunes::Owner{
                    email: vec!["johndoe@example.com".to_string()],
                    name: vec!["John Doe".to_string()],
                }],
                itunes_image: vec![itunes::Image{
                    href: None,
                }],

                item: vec!{
                    Item{
                        title: vec!["Episode 3 - The Future".to_string()],
                        description: vec!["<p>A look into the future of podcasting and how we get to Podcasting 2.0!</p>".to_string()],
                        link: vec![Url::Ok(url::Url::parse("https://example.com/podcast/ep0003").unwrap())],
                        guid: vec![Guid{
                            is_permalink: Some(Bool::Ok(true)),
                            value: Some(GuidValue::Url(url::Url::parse("https://example.com/ep0003").unwrap())),
                        }],
                        pub_date: vec![badpod::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2020, 10, 9).and_hms(4, 30, 38))],
                        enclosure: vec![Enclosure{
                            url: Some(Url::Ok(url::Url::parse("https://example.com/file-03.mp3").unwrap())),
                            length: Some(Integer::Ok(43200000)),
                            type_: Some(MimeEnclosure::AudioMp3),
                        }],

                        itunes_image: vec![itunes::Image{
                            href: None,
                        }],
                        itunes_explicit: vec![Bool::Other(("no".to_string(), "should be \"true\" or \"false\"".to_string()))],

                        podcast_images: vec![podcast::Images {
                            srcset: podcast::ImageSrcSet::Ok(vec![
                                (url::Url::parse("https://example.com/images/ep3/pci_avatar-massive.jpg").unwrap(), 1500),
                                (url::Url::parse("https://example.com/images/ep3/pci_avatar-middle.jpg").unwrap(), 600),
                                (url::Url::parse("https://example.com/images/ep3/pci_avatar-small.jpg").unwrap(), 300),
                                (url::Url::parse("https://example.com/images/ep3/pci_avatar-tiny.jpg").unwrap(), 150),
                            ]),
                        }],
                        podcast_season: vec![podcast::Season{
                            name: Some("Podcasting 2.0".to_string()),
                            value: Some(Integer::Ok(1)),
                        }],
                        podcast_episode: vec![podcast::Episode{
                            display: None,
                            value: Some(Number::Integer(3)),
                        }],
                        podcast_chapters: vec![podcast::Chapters{
                            url: Some(Url::Ok(url::Url::parse("https://example.com/ep3_chapters.json").unwrap())),
                            type_: Some(MimeChapters::ApplicationJson),
                        }],
                        podcast_soundbite: vec!{
                            podcast::Soundbite{
                                start_time: Some(Float::Ok(33.833)),
                                duration: Some(Float::Ok(60.0)),
                                value: None,
                            },
                        },
                        podcast_transcript: vec!{
                            podcast::Transcript{
                                url: Some(Url::Ok(url::Url::parse("https://example.com/ep3/transcript.txt").unwrap())),
                                type_: Some(MimeTranscript::TextPlain),
                                ..Default::default()
                            },
                        },
                        podcast_person: vec!{
                            podcast::Person{
                                href: Some(Url::Ok(url::Url::parse("https://www.podchaser.com/creators/adam-curry-107ZzmWE5f").unwrap())),
                                img: Some(Url::Ok(url::Url::parse("https://example.com/images/adamcurry.jpg").unwrap())),
                                value: Some("Adam Curry".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                role: Some(podcast::PersonRole::Guest),
                                href: Some(Url::Ok(url::Url::parse("https://github.com/daveajones/").unwrap())),
                                img: Some(Url::Ok(url::Url::parse("https://example.com/images/davejones.jpg").unwrap())),
                                value: Some("Dave Jones".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                group: Some(podcast::PersonGroup::Visuals),
                                role: Some(podcast::PersonRole::CoverArtDesigner),
                                href: Some(Url::Ok(url::Url::parse("https://example.com/artist/beckysmith").unwrap())),
                                value: Some("Becky Smith".to_string()),
                                ..Default::default()
                            },
                        },
                        podcast_alternate_enclosure: vec!{
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioMp3),
                                length: Some(Integer::Ok(43200000)),
                                bit_rate: Some(Float::Ok(128000.0)),
                                default: Some(Bool::Ok(true)),
                                title: Some("Standard".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-03.mp3").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomMpegFile03").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioOpus),
                                length: Some(Integer::Ok(32400000)),
                                bit_rate: Some(Float::Ok(96000.0)),
                                title: Some("High quality".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-high-03.opus").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomHighBitrateOpusFile03").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioAac),
                                length: Some(Integer::Ok(54000000)),
                                bit_rate: Some(Float::Ok(160000.0)),
                                title: Some("High quality AAC".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-proprietary-03.aac").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomProprietaryAACFile03").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioOpus),
                                length: Some(Integer::Ok(5400000)),
                                bit_rate: Some(Float::Ok(16000.0)),
                                title: Some("Low bandwidth".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-low-03.opus").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomLowBitrateOpusFile03").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::VideoMp4),
                                length: Some(Integer::Ok(7924786)),
                                bit_rate: Some(Float::Ok(511276.52)),
                                height: Some(Integer::Ok(720)),
                                title: Some("Video version".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-720.mp4").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://QmX33FYehk6ckGQ6g1D9D3FqZPix5JpKstKQKbaS8quUFb").unwrap())),
                                        type_: None,
                                    }
                                },
                                podcast_integrity: vec![podcast::Integrity{
                                    type_: Some(podcast::IntegrityType::Sri),
                                    value: Some("sha384-ExVqijgYHm15PqQqdXfW95x+Rs6C+d6E/ICxyQOeFevnxNLR/wtJNrNYTjIysUBo".to_string()),
                                }],
                                ..Default::default()
                            },
                        },
                        podcast_value: vec![podcast::Value {
                            type_: Some(podcast::ValueType::Lightning),
                            method: Some(podcast::ValueMethod::Keysend),
                            suggested: Some(Float::Ok(0.00000005)),
                            value_recipient: vec! {
                                podcast::ValueRecipient{
                                    name: Some("podcaster".to_string()),
                                    type_: Some(podcast::ValueRecipientType::Node),
                                    address: Some("036557ea56b3b86f08be31bcd2557cae8021b0e3a9413f0c0e52625c6696972e57".to_string()),
                                    split: Some(Integer::Ok(49)),
                                    ..Default::default()
                                },
                                podcast::ValueRecipient{
                                    name: Some("hosting company".to_string()),
                                    type_: Some(podcast::ValueRecipientType::Node),
                                    address: Some("036557ea56b3b86f08be31bcd2557cae8021b0e3a9413f0c0e52625c6696972e57".to_string()),
                                    split: Some(Integer::Ok(1)),
                                    ..Default::default()
                                },
                                podcast::ValueRecipient{
                                    name: Some("Gigi (Guest)".to_string()),
                                    type_: Some(podcast::ValueRecipientType::Node),
                                    address: Some("02e12fea95f576a680ec1938b7ed98ef0855eadeced493566877d404e404bfbf52".to_string()),
                                    split: Some(Integer::Ok(50)),
                                    ..Default::default()
                                },
                            },
                        }],
                        podcast_social_interact: vec!{
                            podcast::SocialInteract{
                                priority: Some(Integer::Ok(1)),
                                uri: Some(Url::Ok(url::Url::parse("https://podcastindex.social/web/@dave/108013847520053258").unwrap())),
                                protocol: Some(podcast::SocialProtocol::ActivityPub),
                                account_id: Some("@dave".to_string()),
                                account_url: Some(Url::Ok(url::Url::parse("https://podcastindex.social/web/@dave").unwrap())),
                            },
                            podcast::SocialInteract{
                                priority: Some(Integer::Ok(2)),
                                uri: Some(Url::Ok(url::Url::parse("https://twitter.com/PodcastindexOrg/status/1507120226361647115").unwrap())),
                                protocol: Some(podcast::SocialProtocol::Twitter),
                                account_id: Some("@podcastindexorg".to_string()),
                                account_url: Some(Url::Ok(url::Url::parse("https://twitter.com/PodcastindexOrg").unwrap())),
                            },
                        },

                        ..Default::default()
                    },
                    Item{
                        title: vec!["Episode 2 - The Present".to_string()],
                        description: vec!["<p>Where are we at now in the podcasting era. What are the current challenges?</p>".to_string()],
                        link: vec![Url::Ok(url::Url::parse("https://example.com/podcast/ep0002").unwrap())],
                        guid: vec![Guid{
                            is_permalink: Some(Bool::Ok(true)),
                            value: Some(GuidValue::Url(url::Url::parse("https://example.com/ep0002").unwrap())),
                        }],
                        pub_date: vec![badpod::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2020, 10, 8).and_hms(4, 30, 38))],
                        enclosure: vec![Enclosure{
                            url: Some(Url::Ok(url::Url::parse("https://example.com/file-02.mp3").unwrap())),
                            length: Some(Integer::Ok(43113000)),
                            type_: Some(MimeEnclosure::AudioMp3),
                        }],

                        itunes_image: vec![itunes::Image{
                            href: None,
                        }],
                        itunes_explicit: vec![Bool::Other(("no".to_string(), "should be \"true\" or \"false\"".to_string()))],
                        podcast_images: vec![podcast::Images {
                            srcset: podcast::ImageSrcSet::Ok(vec![
                                (url::Url::parse("https://example.com/images/ep2/pci_avatar-massive.jpg").unwrap(), 1500),
                                (url::Url::parse("https://example.com/images/ep2/pci_avatar-middle.jpg").unwrap(), 600),
                                (url::Url::parse("https://example.com/images/ep2/pci_avatar-small.jpg").unwrap(), 300),
                                (url::Url::parse("https://example.com/images/ep2/pci_avatar-tiny.jpg").unwrap(), 150),
                            ]),
                        }],

                        podcast_season: vec![podcast::Season{
                            name: Some("Podcasting 2.0".to_string()),
                            value: Some(Integer::Ok(1)),
                        }],
                        podcast_episode: vec![podcast::Episode{
                            display: None,
                            value: Some(Number::Integer(2)),
                        }],
                        podcast_chapters: vec![podcast::Chapters{
                            url: Some(Url::Ok(url::Url::parse("https://example.com/ep2_chapters.json").unwrap())),
                            type_: Some(MimeChapters::ApplicationJson),
                        }],
                        podcast_soundbite: vec!{
                            podcast::Soundbite{
                                start_time: Some(Float::Ok(45.4)),
                                duration: Some(Float::Ok(56.0)),
                                value: None,
                            },
                        },
                        podcast_transcript: vec!{
                            podcast::Transcript{
                                url: Some(Url::Ok(url::Url::parse("https://example.com/ep2/transcript.txt").unwrap())),
                                type_: Some(MimeTranscript::TextPlain),
                                ..Default::default()
                            },
                        },
                        podcast_person: vec!{
                            podcast::Person{
                                href: Some(Url::Ok(url::Url::parse("https://en.wikipedia.org/wiki/Adam_Curry").unwrap())),
                                img: Some(Url::Other(("http://example.com/images/adamcurry.jpg".to_string(), "protocol must be `https`".to_string()))),
                                value: Some("Adam Curry".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                role: Some(podcast::PersonRole::Guest),
                                href: Some(Url::Ok(url::Url::parse("https://example.com/blog/daveajones/").unwrap())),
                                img: Some(Url::Other(("http://example.com/images/davejones.jpg".to_string(), "protocol must be `https`".to_string()))),
                                value: Some("Dave Jones".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                group: Some(podcast::PersonGroup::Visuals),
                                role: Some(podcast::PersonRole::CoverArtDesigner),
                                href: Some(Url::Ok(url::Url::parse("https://example.com/artist/marcusbrown").unwrap())),
                                value: Some("Marcus Brown".to_string()),
                                ..Default::default()
                            },
                        },
                        podcast_alternate_enclosure: vec!{
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioMp3),
                                length: Some(Integer::Ok(43200000)),
                                bit_rate: Some(Float::Ok(128000.0)),
                                default: Some(Bool::Ok(true)),
                                title: Some("Standard".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-02.mp3").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomMpegFile02").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioOpus),
                                length: Some(Integer::Ok(32400000)),
                                bit_rate: Some(Float::Ok(96000.0)),
                                title: Some("High quality".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-high-02.opus").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomHighBitrateOpusFile02").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioAac),
                                length: Some(Integer::Ok(54000000)),
                                bit_rate: Some(Float::Ok(160000.0)),
                                title: Some("High quality AAC".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-proprietary-02.aac").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomProprietaryAACFile02").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioOpus),
                                length: Some(Integer::Ok(5400000)),
                                bit_rate: Some(Float::Ok(16000.0)),
                                title: Some("Low bandwidth".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-low-02.opus").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomLowBitrateOpusFile02").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                        },
                        podcast_value: vec![podcast::Value {
                            type_: Some(podcast::ValueType::Lightning),
                            method: Some(podcast::ValueMethod::Keysend),
                            suggested: Some(Float::Ok(0.00000005)),
                            value_recipient: vec! {
                                podcast::ValueRecipient{
                                    name: Some("podcaster".to_string()),
                                    type_: Some(podcast::ValueRecipientType::Node),
                                    address: Some("036557ea56b3b86f08be31bcd2557cae8021b0e3a9413f0c0e52625c6696972e57".to_string()),
                                    split: Some(Integer::Ok(49)),
                                    ..Default::default()
                                },
                                podcast::ValueRecipient{
                                    name: Some("hosting company".to_string()),
                                    type_: Some(podcast::ValueRecipientType::Node),
                                    address: Some("036557ea56b3b86f08be31bcd2557cae8021b0e3a9413f0c0e52625c6696972e57".to_string()),
                                    split: Some(Integer::Ok(1)),
                                    ..Default::default()
                                },
                                podcast::ValueRecipient{
                                    name: Some("Paul Itoi (Guest)".to_string()),
                                    type_: Some(podcast::ValueRecipientType::Node),
                                    address: Some("03a9a8d953fe747d0dd94dd3c567ddc58451101e987e2d2bf7a4d1e10a2c89ff38".to_string()),
                                    split: Some(Integer::Ok(50)),
                                    ..Default::default()
                                },
                            },
                        }],
                        ..Default::default()
                    },
                    Item{
                        title: vec!["Episode 1 - The Past".to_string()],
                        description: vec!["<p>How did podcasting get started? What was it like in the beginning?</p>".to_string()],
                        link: vec![Url::Ok(url::Url::parse("https://example.com/podcast/ep0001").unwrap())],
                        guid: vec![Guid{
                            is_permalink: Some(Bool::Ok(true)),
                            value: Some(GuidValue::Url(url::Url::parse("https://example.com/ep0001").unwrap())),
                        }],
                        pub_date: vec![badpod::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2020, 10, 7).and_hms(4, 30, 38))],
                        enclosure: vec![Enclosure{
                            url: Some(Url::Ok(url::Url::parse("https://example.com/file-01.mp3").unwrap())),
                            length: Some(Integer::Ok(43111403)),
                            type_: Some(MimeEnclosure::AudioMp3),
                        }],

                        itunes_image: vec![itunes::Image{
                            href: None,
                        }],
                        itunes_explicit: vec![Bool::Other(("no".to_string(), "should be \"true\" or \"false\"".to_string()))],
                        podcast_images: vec![podcast::Images {
                            srcset: podcast::ImageSrcSet::Ok(vec![
                                (url::Url::parse("https://example.com/images/ep1/pci_avatar-massive.jpg").unwrap(), 1500),
                                (url::Url::parse("https://example.com/images/ep1/pci_avatar-middle.jpg").unwrap(), 600),
                                (url::Url::parse("https://example.com/images/ep1/pci_avatar-small.jpg").unwrap(), 300),
                                (url::Url::parse("https://example.com/images/ep1/pci_avatar-tiny.jpg").unwrap(), 150),
                            ]),
                        }],
                        podcast_season: vec![podcast::Season{
                            name: Some("Podcasting 2.0".to_string()),
                            value: Some(Integer::Ok(1)),
                        }],
                        podcast_episode: vec![podcast::Episode{
                            display: None,
                            value: Some(Number::Integer(1)),
                        }],
                        podcast_chapters: vec![podcast::Chapters{
                            url: Some(Url::Ok(url::Url::parse("https://example.com/ep1_chapters.json").unwrap())),
                            type_: Some(MimeChapters::ApplicationJson),
                        }],
                        podcast_soundbite: vec!{
                            podcast::Soundbite{
                                start_time: Some(Float::Ok(29.32)),
                                duration: Some(Float::Ok(34.0)),
                                value: None,
                            },
                        },
                        podcast_transcript: vec!{
                            podcast::Transcript{
                                url: Some(Url::Ok(url::Url::parse("https://example.com/ep1/transcript.txt").unwrap())),
                                type_: Some(MimeTranscript::TextPlain),
                                ..Default::default()
                            },
                        },
                        podcast_person: vec!{
                            podcast::Person{
                                href: Some(Url::Ok(url::Url::parse("https://curry.com").unwrap())),
                                img: Some(Url::Other(("http://example.com/images/adamcurry.jpg".to_string(), "protocol must be `https`".to_string()))),
                                value: Some("Adam Curry".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                role: Some(podcast::PersonRole::Guest),
                                href: Some(Url::Ok(url::Url::parse("https://www.imdb.com/name/nm0427852888/").unwrap())),
                                img: Some(Url::Other(("http://example.com/images/davejones.jpg".to_string(), "protocol must be `https`".to_string()))),
                                value: Some("Dave Jones".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                group: Some(podcast::PersonGroup::Visuals),
                                role: Some(podcast::PersonRole::CoverArtDesigner),
                                href: Some(Url::Ok(url::Url::parse("https://example.com/artist/jebickmorton").unwrap())),
                                value: Some("Jebick Morton".to_string()),
                                ..Default::default()
                            },
                        },
                        podcast_alternate_enclosure: vec!{
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioMp3),
                                length: Some(Integer::Ok(43203200)),
                                bit_rate: Some(Float::Ok(128000.0)),
                                default: Some(Bool::Ok(true)),
                                title: Some("Standard".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-01.mp3").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomMpegFile01").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioOpus),
                                length: Some(Integer::Ok(32406000)),
                                bit_rate: Some(Float::Ok(96000.0)),
                                title: Some("High quality".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-high-01.opus").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomHighBitrateOpusFile01").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioAac),
                                length: Some(Integer::Ok(5400300)),
                                bit_rate: Some(Float::Ok(160000.0)),
                                title: Some("High quality AAC".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-proprietary-01.aac").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomProprietaryAACFile01").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::AudioOpus),
                                length: Some(Integer::Ok(5042000)),
                                bit_rate: Some(Float::Ok(16000.0)),
                                title: Some("Low bandwidth".to_string()),
                                podcast_source: vec!{
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("https://example.com/file-low-01.opus").unwrap())),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some(Url::Ok(url::Url::parse("ipfs://someRandomLowBitrateOpusFile01").unwrap())),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    },
                },

                ..Default::default()
            }],
        })
    ),
    ];

    for (input, expected) in conditions {
        let output = badpod::from_str(input);
        match (output, expected) {
            (Ok(output), Ok(expected)) => {
                pretty_assertions::assert_eq!(output, *expected);
            }
            (Err(output), Err(expected)) => {
                pretty_assertions::assert_eq!(output, *expected);
            }
            (Err(output), Ok(_)) => {
                panic!("Unexpected error: {:?}", output);
            }
            (Ok(output), Err(_)) => {
                panic!("Unexpected success: {:?}", output);
            }
        }
    }
}

// Ensures that example feeds from a number of different hosting companies can be parsed.
#[test]
fn no_error() {
    struct Test {
        url: &'static str,
        title: &'static str,
    }

    let tests = vec![
        // Simplecast
        Test{url: "https://feeds.simplecast.com/54nAGcIl", title: "The Daily"},
        // Megaphone
        Test{url: "https://feeds.megaphone.fm/hubermanlab", title: "Huberman Lab"},
        // NPR
        Test{url: "https://feeds.npr.org/500005/podcast.xml", title: "NPR News Now"},
        // Buzzsprout
        Test{url: "https://feeds.buzzsprout.com/424075.rss", title: "Bret Weinstein | DarkHorse Podcast"},
        // Blubrry/PowerPress - Not working
        Test{url: "https://lexfridman.com/feed/podcast/", title: "Lex Fridman Podcast"},
    ];

    for test in tests {
        let feed_str = match reqwest::blocking::get(test.url) {
            Ok(response) => response.text(),
            Err(_) => {
                log::warn!("Failed to fetch feed: {}", test.url);
                continue;
            },
        };
        let feed_str = match feed_str {
            Ok(feed_str) => feed_str,
            Err(_) => {
                log::warn!("Failed to read the feed: {}", test.url);
                continue;
            },
        };
        let feed = badpod::from_str(&feed_str);
        pretty_assertions::assert_eq!(test.title, feed.unwrap().channel[0].title[0]);
    }
}
