use chrono::prelude::*;
use parse_rss::*;

#[test]
fn deserialize() {
    let conditions : &[(&str, Result<Rss,String>)] = &[
        (
            include_str!("data/test_feed.xml"),
        Ok(Rss {
            version: Some("2.0".to_string()),
            channel: Some(Channel {
                copyright: Some("© Example Company".to_string()),
                description: Some("<p><strong>Example HTML description</strong></p>".to_string()),
                language: Some(Language::Lithuanian),
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
                    osm: Some(podcast::Osm::Ok(podcast::OsmObject {
                        type_: podcast::OsmType::Relation,
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
                        type_: Some(MimeEnclosure::Mp3),
                        season: None,
                        value: Some("Coming April 1st, 2021".to_string()),
                    },
                },
                podcast_license: Some(podcast::License {
                    url: None,
                    value: Some(podcast::LicenseType::CreativeCommonsAttribution4_0International),
                }),
                podcast_guid: Some(podcast::Guid::Ok(
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
                        type_: Some(MimeEnclosure::Mp3),
                    }),
                    itunes_duration: Some(Number::Integer(1079)),
                    itunes_explicit: Some(Bool::Ok(true)),
                    pub_date: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2022, 10, 10).and_hms(6, 10, 5))),

                    podcast_chapters: Some(podcast::Chapters{
                        url: Some("https://example.com/episode-1/chapters.json".to_string()),
                        type_: Some(MimeChapters::ApplicationJsonChapters),
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
                        osm: Some(podcast::Osm::Ok(podcast::OsmObject {
                            type_: podcast::OsmType::Way,
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
                            type_: Some(MimeTranscript::Json),
                            language: Some(Language::Spanish(LanguageSpanish::Default)),
                            rel: Some(podcast::TranscriptRel::Captions),
                        },
                    },
                    itunes_block: Some(itunes::Yes::Other("yes".to_string())),
                    podcast_alternate_enclosures: vec!{
                        podcast::AlternateEnclosure{
                            type_: Some(MimeEnclosure::Mp3),
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
                                    type_: Some(MimeEnclosure::Other("application/x-bittorrent".to_string())),
                                },
                                podcast::Source{
                                    uri: Some("http://example.onion/file-0.mp3".to_string()),
                                    type_: None,
                                },
                            },
                            ..Default::default()
                        },
                        podcast::AlternateEnclosure{
                            type_: Some(MimeEnclosure::Mp4),
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
                        guid: Some(Guid{
                            is_permalink: None,
                            value: Some("e32b4890-983b-4ce5-8b46-f2d6bc1d8819".to_string()),
                        }),
                        enclosure: Some(Enclosure{
                            url: Some("https://example.com/pc20/livestream?format=.mp3".to_string()),
                            length: Some(Integer::Ok(312)),
                            type_: Some(MimeEnclosure::Mp3),
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
        }),
    ),
    (
        include_str!("data/podcast_namespace_example.xml"),
        Ok(Rss{
            version: Some("2.0".to_string()),
            channel: Some(Channel{
                title: Some("Podcasting 2.0 Namespace Example".to_string()),
                description: Some("This is a fake show that exists only as an example of the \"podcast\" namespace tag usage.".to_string()),
                link: Some("http://example.com/podcast".to_string()),
                language: Some(Language::English(LanguageEnglish::UnitedStates)),
                generator: Some("Freedom Controller".to_string()),

                podcast_guid: Some(podcast::Guid::Other("y0ur-gu1d-g035-h3r3".to_string())),
                podcast_license: Some(podcast::License{
                    url: Some("https://example.org/mypodcastlicense/full.pdf".to_string()),
                    value: Some(podcast::LicenseType::Other("my-podcast-license-v1".to_string())),
                }),
                podcast_locked: Some(podcast::Locked{
                    owner: Some("podcastowner@example.com".to_string()),
                    value: Some(Bool::Ok(true)),
                }),
                podcast_blocks: vec! {
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
                podcast_fundings: vec! {
                    podcast::Funding{
                        url: Some("https://example.com/donate".to_string()),
                        value: Some("Support the show!".to_string()),
                    },
                },
                podcast_location: Some(podcast::Location{
                    geo: Some(podcast::Geo::Ok(podcast::GeoCoordinates{
                        latitude: 30.2672,
                        longitude: 97.7431,
                        altitude: None,
                        uncertainty: None,
                    })),
                    osm: Some(podcast::Osm::Ok(podcast::OsmObject{
                        type_: podcast::OsmType::Relation,
                        id: 113314,
                        revision: None,
                    })),
                    value: Some("Austin, TX".to_string()),
                }),
                podcast_medium: Some(podcast::Medium::Podcast),
                podcast_value: Some(podcast::Value { 
                    type_: Some(podcast::ValueType::Lightning),
                    method: Some(podcast::ValueMethod::Keysend),
                    suggested: Some(Float::Ok(0.00000005)),
                    value_recipients: vec!{
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
                }),
                podcast_trailers: vec!{
                    podcast::Trailer{
                        url: Some("https://example.org/trailers/teaser".to_string()),
                        pub_date: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(5*60*60).ymd(2021, 4, 1).and_hms(8, 0, 0))),
                        length: Some(Integer::Ok(12345678)),
                        type_: Some(MimeEnclosure::Other("audio/mp3".to_string())),
                        value: Some("Coming April 1st, 2021".to_string()),
                        season: None,
                    },
                },
                podcast_live_items: vec!{
                    podcast::LiveItem{
                        status: Some(podcast::LiveItemStatus::Live),
                        start: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(6*60*60).ymd(2021, 9, 26).and_hms(7, 30, 0))),
                        end: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(6*60*60).ymd(2021, 9, 26).and_hms(9, 30, 0))),
                        title: Some("Podcasting 2.0 Live Show".to_string()),
                        description: Some("A look into the future of podcasting and how we get to Podcasting 2.0!".to_string()),
                        link: Some("https://example.com/podcast/live".to_string()),
                        guid: Some(Guid{
                            is_permalink: Some(Bool::Ok(true)),
                            value: Some("https://example.com/live".to_string()),
                        }),
                        podcast_alternate_enclosures: vec!{
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Mp3),
                                length: Some(Integer::Ok(312)),
                                default: Some(Bool::Ok(true)),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/pc20/livestream".to_string()),
                                        type_: None,
                                    },
                                },
                                ..Default::default()
                            },
                        },
                        enclosure: Some(Enclosure {
                            url: Some("https://example.com/pc20/livestream?format=.mp3".to_string()),
                            type_: Some(MimeEnclosure::Mp3),
                            length: Some(Integer::Ok(312)),
                        }),
                        content_links: vec!{
                            podcast::ContentLink{
                                href: Some("https://youtube.com/pc20/livestream".to_string()),
                                value: Some("YouTube!".to_string()),
                            },
                            podcast::ContentLink{
                                href: Some("https://twitch.com/pc20/livestream".to_string()),
                                value: Some("Twitch!".to_string()),
                            },
                        },
                        ..Default::default()
                    },
                },

                itunes_author: Some("John Doe".to_string()),
                itunes_explicit: Some(Bool::Other("no".to_string())),
                itunes_type: Some(itunes::PodcastType::Episodic),
                itunes_categories: vec!{
                    itunes::Category{
                        text: Some(itunes::CategoryName::News),
                        subcategory: None,
                    },
                    itunes::Category{
                        text: Some(itunes::CategoryName::Technology),
                        subcategory: None,
                    },
                },
                itunes_owner: Some(itunes::Owner{
                    email: Some("johndoe@example.com".to_string()),
                    name: Some("John Doe".to_string()),
                }),
                itunes_image: Some(itunes::Image{
                    href: None,
                }),

                items: vec!{
                    Item{
                        title: Some("Episode 3 - The Future".to_string()),
                        description: Some("<p>A look into the future of podcasting and how we get to Podcasting 2.0!</p>".to_string()),
                        link: Some("https://example.com/podcast/ep0003".to_string()),
                        guid: Some(Guid{
                            is_permalink: Some(Bool::Ok(true)),
                            value: Some("https://example.com/ep0003".to_string()),
                        }),
                        pub_date: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2020, 10, 9).and_hms(4, 30, 38))),
                        enclosure: Some(Enclosure{
                            url: Some("https://example.com/file-03.mp3".to_string()),
                            length: Some(Integer::Ok(43200000)),
                            type_: Some(MimeEnclosure::Mp3),
                        }),

                        itunes_image: Some(itunes::Image{
                            href: None,
                        }),
                        itunes_explicit: Some(Bool::Other("no".to_string())),

                        podcast_images: Some(podcast::Images {
                            srcset: vec! {
                                podcast::Image::Ok("https://example.com/images/ep3/pci_avatar-massive.jpg".to_string(), 1500),
                                podcast::Image::Ok("https://example.com/images/ep3/pci_avatar-middle.jpg".to_string(), 600),
                                podcast::Image::Ok("https://example.com/images/ep3/pci_avatar-small.jpg".to_string(), 300),
                                podcast::Image::Ok("https://example.com/images/ep3/pci_avatar-tiny.jpg".to_string(), 150),
                            },
                        }),
                        podcast_season: Some(podcast::Season{
                            name: Some("Podcasting 2.0".to_string()),
                            value: Some(Integer::Ok(1)),
                        }),
                        podcast_episode: Some(podcast::Episode{
                            display: None,
                            value: Some(Number::Integer(3)),
                        }),
                        podcast_chapters: Some(podcast::Chapters{
                            url: Some("https://example.com/ep3_chapters.json".to_string()),
                            type_: Some(MimeChapters::ApplicationJson),
                        }),
                        podcast_soundbites: vec!{
                            podcast::Soundbite{
                                start_time: Some(Float::Ok(33.833)),
                                duration: Some(Float::Ok(60.0)),
                                value: None,
                            },
                        },
                        podcast_transcripts: vec!{
                            podcast::Transcript{
                                url: Some("https://example.com/ep3/transcript.txt".to_string()),
                                type_: Some(MimeTranscript::Plain),
                                ..Default::default()
                            },
                        },
                        podcast_persons: vec!{
                            podcast::Person{
                                href: Some("https://www.podchaser.com/creators/adam-curry-107ZzmWE5f".to_string()),
                                img: Some("https://example.com/images/adamcurry.jpg".to_string()),
                                value: Some("Adam Curry".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                role: Some(podcast::PersonRole::Guest),
                                href: Some("https://github.com/daveajones/".to_string()),
                                img: Some("https://example.com/images/davejones.jpg".to_string()),
                                value: Some("Dave Jones".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                group: Some(podcast::PersonGroup::Visuals),
                                role: Some(podcast::PersonRole::CoverArtDesigner),
                                href: Some("https://example.com/artist/beckysmith".to_string()),
                                value: Some("Becky Smith".to_string()),
                                ..Default::default()
                            },
                        },
                        podcast_alternate_enclosures: vec!{
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Mp3),
                                length: Some(Integer::Ok(43200000)),
                                bit_rate: Some(Float::Ok(128000.0)),
                                default: Some(Bool::Ok(true)),
                                title: Some("Standard".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-03.mp3".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomMpegFile03".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Other("audio/opus".to_string())),
                                length: Some(Integer::Ok(32400000)),
                                bit_rate: Some(Float::Ok(96000.0)),
                                title: Some("High quality".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-high-03.opus".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomHighBitrateOpusFile03".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Other("audio/aac".to_string())),
                                length: Some(Integer::Ok(54000000)),
                                bit_rate: Some(Float::Ok(160000.0)),
                                title: Some("High quality AAC".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-proprietary-03.aac".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomProprietaryAACFile03".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Other("audio/opus".to_string())),
                                length: Some(Integer::Ok(5400000)),
                                bit_rate: Some(Float::Ok(16000.0)),
                                title: Some("Low bandwidth".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-low-03.opus".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomLowBitrateOpusFile03".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Mp4),
                                length: Some(Integer::Ok(7924786)),
                                bit_rate: Some(Float::Ok(511276.52)),
                                height: Some(Integer::Ok(720)),
                                title: Some("Video version".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-720.mp4".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://QmX33FYehk6ckGQ6g1D9D3FqZPix5JpKstKQKbaS8quUFb".to_string()),
                                        type_: None,
                                    }
                                },
                                podcast_integrity: Some(podcast::Integrity{
                                    type_: Some(podcast::IntegrityType::Sri),
                                    value: Some("sha384-ExVqijgYHm15PqQqdXfW95x+Rs6C+d6E/ICxyQOeFevnxNLR/wtJNrNYTjIysUBo".to_string()),
                                }),
                                ..Default::default()
                            },
                        },
                        podcast_value: Some(podcast::Value {
                            type_: Some(podcast::ValueType::Lightning),
                            method: Some(podcast::ValueMethod::Keysend),
                            suggested: Some(Float::Ok(0.00000005)),
                            value_recipients: vec! {
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
                        }),
                        podcast_social_interacts: vec!{
                            podcast::SocialInteract{
                                priority: Some(Integer::Ok(1)),
                                uri: Some("https://podcastindex.social/web/@dave/108013847520053258".to_string()),
                                protocol: Some(podcast::SocialProtocol::ActivityPub),
                                account_id: Some("@dave".to_string()),
                                account_url: Some("https://podcastindex.social/web/@dave".to_string()),
                            },
                            podcast::SocialInteract{
                                priority: Some(Integer::Ok(2)),
                                uri: Some("https://twitter.com/PodcastindexOrg/status/1507120226361647115".to_string()),
                                protocol: Some(podcast::SocialProtocol::Twitter),
                                account_id: Some("@podcastindexorg".to_string()),
                                account_url: Some("https://twitter.com/PodcastindexOrg".to_string()),
                            },
                        },

                        ..Default::default()
                    },
                    Item{
                        title: Some("Episode 2 - The Present".to_string()),
                        description: Some("<p>Where are we at now in the podcasting era. What are the current challenges?</p>".to_string()),
                        link: Some("https://example.com/podcast/ep0002".to_string()),
                        guid: Some(Guid{
                            is_permalink: Some(Bool::Ok(true)),
                            value: Some("https://example.com/ep0002".to_string()),
                        }),
                        pub_date: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2020, 10, 8).and_hms(4, 30, 38))),
                        enclosure: Some(Enclosure{
                            url: Some("https://example.com/file-02.mp3".to_string()),
                            length: Some(Integer::Ok(43113000)),
                            type_: Some(MimeEnclosure::Mp3),
                        }),

                        itunes_image: Some(itunes::Image{
                            href: None,
                        }),
                        itunes_explicit: Some(Bool::Other("no".to_string())),

                        podcast_images: Some(podcast::Images {
                            srcset: vec! {
                                podcast::Image::Ok("https://example.com/images/ep2/pci_avatar-massive.jpg".to_string(), 1500),
                                podcast::Image::Ok("https://example.com/images/ep2/pci_avatar-middle.jpg".to_string(), 600),
                                podcast::Image::Ok("https://example.com/images/ep2/pci_avatar-small.jpg".to_string(), 300),
                                podcast::Image::Ok("https://example.com/images/ep2/pci_avatar-tiny.jpg".to_string(), 150),
                            },
                        }),
                        podcast_season: Some(podcast::Season{
                            name: Some("Podcasting 2.0".to_string()),
                            value: Some(Integer::Ok(1)),
                        }),
                        podcast_episode: Some(podcast::Episode{
                            display: None,
                            value: Some(Number::Integer(2)),
                        }),
                        podcast_chapters: Some(podcast::Chapters{
                            url: Some("https://example.com/ep2_chapters.json".to_string()),
                            type_: Some(MimeChapters::ApplicationJson),
                        }),
                        podcast_soundbites: vec!{
                            podcast::Soundbite{
                                start_time: Some(Float::Ok(45.4)),
                                duration: Some(Float::Ok(56.0)),
                                value: None,
                            },
                        },
                        podcast_transcripts: vec!{
                            podcast::Transcript{
                                url: Some("https://example.com/ep2/transcript.txt".to_string()),
                                type_: Some(MimeTranscript::Plain),
                                ..Default::default()
                            },
                        },
                        podcast_persons: vec!{
                            podcast::Person{
                                href: Some("https://en.wikipedia.org/wiki/Adam_Curry".to_string()),
                                img: Some("http://example.com/images/adamcurry.jpg".to_string()),
                                value: Some("Adam Curry".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                role: Some(podcast::PersonRole::Guest),
                                href: Some("https://example.com/blog/daveajones/".to_string()),
                                img: Some("http://example.com/images/davejones.jpg".to_string()),
                                value: Some("Dave Jones".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                group: Some(podcast::PersonGroup::Visuals),
                                role: Some(podcast::PersonRole::CoverArtDesigner),
                                href: Some("https://example.com/artist/marcusbrown".to_string()),
                                value: Some("Marcus Brown".to_string()),
                                ..Default::default()
                            },
                        },
                        podcast_alternate_enclosures: vec!{
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Mp3),
                                length: Some(Integer::Ok(43200000)),
                                bit_rate: Some(Float::Ok(128000.0)),
                                default: Some(Bool::Ok(true)),
                                title: Some("Standard".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-02.mp3".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomMpegFile02".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Other("audio/opus".to_string())),
                                length: Some(Integer::Ok(32400000)),
                                bit_rate: Some(Float::Ok(96000.0)),
                                title: Some("High quality".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-high-02.opus".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomHighBitrateOpusFile02".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Other("audio/aac".to_string())),
                                length: Some(Integer::Ok(54000000)),
                                bit_rate: Some(Float::Ok(160000.0)),
                                title: Some("High quality AAC".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-proprietary-02.aac".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomProprietaryAACFile02".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Other("audio/opus".to_string())),
                                length: Some(Integer::Ok(5400000)),
                                bit_rate: Some(Float::Ok(16000.0)),
                                title: Some("Low bandwidth".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-low-02.opus".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomLowBitrateOpusFile02".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                        },
                        podcast_value: Some(podcast::Value {
                            type_: Some(podcast::ValueType::Lightning),
                            method: Some(podcast::ValueMethod::Keysend),
                            suggested: Some(Float::Ok(0.00000005)),
                            value_recipients: vec! {
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
                        }),
                        ..Default::default()
                    },
                    Item{
                        title: Some("Episode 1 - The Past".to_string()),
                        description: Some("<p>How did podcasting get started? What was it like in the beginning?</p>".to_string()),
                        link: Some("https://example.com/podcast/ep0001".to_string()),
                        guid: Some(Guid{
                            is_permalink: Some(Bool::Ok(true)),
                            value: Some("https://example.com/ep0001".to_string()),
                        }),
                        pub_date: Some(parse_rss::DateTime::Ok(chrono::FixedOffset::west(0).ymd(2020, 10, 7).and_hms(4, 30, 38))),
                        enclosure: Some(Enclosure{
                            url: Some("https://example.com/file-01.mp3".to_string()),
                            length: Some(Integer::Ok(43111403)),
                            type_: Some(MimeEnclosure::Mp3),
                        }),

                        itunes_image: Some(itunes::Image{
                            href: None,
                        }),
                        itunes_explicit: Some(Bool::Other("no".to_string())),

                        podcast_images: Some(podcast::Images {
                            srcset: vec! {
                                podcast::Image::Ok("https://example.com/images/ep1/pci_avatar-massive.jpg".to_string(), 1500),
                                podcast::Image::Ok("https://example.com/images/ep1/pci_avatar-middle.jpg".to_string(), 600),
                                podcast::Image::Ok("https://example.com/images/ep1/pci_avatar-small.jpg".to_string(), 300),
                                podcast::Image::Ok("https://example.com/images/ep1/pci_avatar-tiny.jpg".to_string(), 150),
                            },
                        }),
                        podcast_season: Some(podcast::Season{
                            name: Some("Podcasting 2.0".to_string()),
                            value: Some(Integer::Ok(1)),
                        }),
                        podcast_episode: Some(podcast::Episode{
                            display: None,
                            value: Some(Number::Integer(1)),
                        }),
                        podcast_chapters: Some(podcast::Chapters{
                            url: Some("https://example.com/ep1_chapters.json".to_string()),
                            type_: Some(MimeChapters::ApplicationJson),
                        }),
                        podcast_soundbites: vec!{
                            podcast::Soundbite{
                                start_time: Some(Float::Ok(29.32)),
                                duration: Some(Float::Ok(34.0)),
                                value: None,
                            },
                        },
                        podcast_transcripts: vec!{
                            podcast::Transcript{
                                url: Some("https://example.com/ep1/transcript.txt".to_string()),
                                type_: Some(MimeTranscript::Plain),
                                ..Default::default()
                            },
                        },
                        podcast_persons: vec!{
                            podcast::Person{
                                href: Some("https://curry.com".to_string()),
                                img: Some("http://example.com/images/adamcurry.jpg".to_string()),
                                value: Some("Adam Curry".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                role: Some(podcast::PersonRole::Guest),
                                href: Some("https://www.imdb.com/name/nm0427852888/".to_string()),
                                img: Some("http://example.com/images/davejones.jpg".to_string()),
                                value: Some("Dave Jones".to_string()),
                                ..Default::default()
                            },
                            podcast::Person{
                                group: Some(podcast::PersonGroup::Visuals),
                                role: Some(podcast::PersonRole::CoverArtDesigner),
                                href: Some("https://example.com/artist/jebickmorton".to_string()),
                                value: Some("Jebick Morton".to_string()),
                                ..Default::default()
                            },
                        },
                        podcast_alternate_enclosures: vec!{
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Mp3),
                                length: Some(Integer::Ok(43203200)),
                                bit_rate: Some(Float::Ok(128000.0)),
                                default: Some(Bool::Ok(true)),
                                title: Some("Standard".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-01.mp3".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomMpegFile01".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Other("audio/opus".to_string())),
                                length: Some(Integer::Ok(32406000)),
                                bit_rate: Some(Float::Ok(96000.0)),
                                title: Some("High quality".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-high-01.opus".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomHighBitrateOpusFile01".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Other("audio/aac".to_string())),
                                length: Some(Integer::Ok(5400300)),
                                bit_rate: Some(Float::Ok(160000.0)),
                                title: Some("High quality AAC".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-proprietary-01.aac".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomProprietaryAACFile01".to_string()),
                                        type_: None,
                                    }
                                },
                                ..Default::default()
                            },
                            podcast::AlternateEnclosure{
                                type_: Some(MimeEnclosure::Other("audio/opus".to_string())),
                                length: Some(Integer::Ok(5042000)),
                                bit_rate: Some(Float::Ok(16000.0)),
                                title: Some("Low bandwidth".to_string()),
                                podcast_sources: vec!{
                                    podcast::Source{
                                        uri: Some("https://example.com/file-low-01.opus".to_string()),
                                        type_: None,
                                    },
                                    podcast::Source{
                                        uri: Some("ipfs://someRandomLowBitrateOpusFile01".to_string()),
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
            }),
        }),
        ),
        (
            include_str!("data/empty_feed.xml"),
            Ok(Rss{
                ..Default::default()
            }),
            ),
    ];

    for (input, expected) in conditions {
        let output = parse_rss::from_str(input);
        pretty_assertions::assert_eq!(output, *expected);
    }
}
