use crate::basic::{Bool, BoolType, Float, Integer, Number, NumberConstraint};
use crate::itunes;
use crate::language::Language;
use crate::mime;
use crate::podcast;
use crate::rss;
use crate::strings::{Url, UrlConstraint};
use crate::time::{DateTime, TimeFormat};

/// Denotes values that cannot be deserialized and the reason for deserialization failure.
pub type Other = (String, String);

const NS_ITUNES: &str = "http://www.itunes.com/dtds/podcast-1.0.dtd";
const NS_CONTENT: &str = "http://purl.org/rss/1.0/modules/content/";
const NS_PODCAST_1: &str =
    "https://github.com/Podcastindex-org/podcast-namespace/blob/main/docs/1.0.md";
const NS_PODCAST_2: &str = "https://podcastindex.org/namespace/1.0";

#[derive(Debug, PartialEq)]
pub enum Error {
    NoRoot,
    RootNotRss,
    Custom(String),
}

/// Converts contents of an XML file of podcast's RSS feed to [Rss](crate::Rss) struct.
pub fn from_str(feed_str: &str) -> Result<rss::Rss, Error> {
    let tree = match roxmltree::Document::parse(feed_str) {
        Ok(tree) => tree,
        Err(roxmltree::Error::NoRootNode) => return Err(Error::NoRoot),
        Err(e) => return Err(Error::Custom(e.to_string())),
    };

    let root = tree.root_element();
    if root.tag_name().name() != "rss" {
        return Err(Error::RootNotRss);
    }

    let mut feed = rss::Rss {
        ..Default::default()
    };
    for attribute in root.attributes() {
        if attribute.name() == "version" {
            feed.version = Some(attribute.value().to_string());
        }
    }
    for child in root.children() {
        if child.tag_name().name() == "channel" {
            feed.channel.push(parse_channel(child));
        }
    }

    Ok(feed)
}

fn parse_channel(channel: roxmltree::Node) -> rss::Channel {
    let mut new_channel = rss::Channel {
        ..Default::default()
    };

    for child in channel.children() {
        let namespace = child.tag_name().namespace();
        let tag_name = child.tag_name().name();
        match (namespace, tag_name) {
            (None, "category") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.category.push(text);
                }
            }
            (None, "copyright") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.copyright.push(text);
                }
            }
            (None, "description") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.description.push(text);
                }
            }
            (None, "generator") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.generator.push(text);
                }
            }
            (None, "item") => {
                new_channel.item.push(parse_item(child));
            }
            (None, "language") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.language.push(Language::parse(&text));
                }
            }
            (None, "lastBuildDate") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel
                        .last_build_date
                        .push(DateTime::parse(&text, TimeFormat::Rfc2822));
                }
            }
            (None, "link") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel
                        .link
                        .push(Url::parse(&text, UrlConstraint::HttpOrHttps));
                }
            }
            (None, "managingEditor") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.managing_editor.push(text);
                }
            }
            (None, "pubDate") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel
                        .pub_date
                        .push(DateTime::parse(&text, TimeFormat::Rfc2822));
                }
            }
            (None, "title") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.title.push(text);
                }
            }
            (None, "ttl") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel
                        .ttl
                        .push(Integer::parse(&text, NumberConstraint::Positive));
                }
            }
            (None, "webMaster") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.web_master.push(text);
                }
            }

            (Some(NS_CONTENT), "encoded") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.content_encoded.push(text);
                }
            }

            (Some(NS_ITUNES), "author") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.itunes_author.push(text);
                }
            }
            (Some(NS_ITUNES), "block") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.itunes_block.push(itunes::Yes::parse(&text));
                }
            }
            (Some(NS_ITUNES), "category") => {
                new_channel
                    .itunes_category
                    .push(parse_itunes_category(child));
            }
            (Some(NS_ITUNES), "complete") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.itunes_complete.push(itunes::Yes::parse(&text));
                }
            }
            (Some(NS_ITUNES), "explicit") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel
                        .itunes_explicit
                        .push(Bool::parse(&text, BoolType::TrueFalse));
                }
            }
            (Some(NS_ITUNES), "image") => {
                new_channel.itunes_image.push(parse_itunes_image(child));
            }
            (Some(NS_ITUNES), "new-feed-url") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel
                        .itunes_new_feed_url
                        .push(Url::parse(&text, UrlConstraint::HttpOrHttps));
                }
            }
            (Some(NS_ITUNES), "owner") => {
                new_channel.itunes_owner.push(parse_itunes_owner(child));
            }
            (Some(NS_ITUNES), "type") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel
                        .itunes_type
                        .push(itunes::PodcastType::parse(&text));
                }
            }

            (Some(NS_PODCAST_1 | NS_PODCAST_2), "locked") => {
                new_channel.podcast_locked.push(parse_podcast_locked(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "funding") => {
                new_channel
                    .podcast_funding
                    .push(parse_podcast_funding(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "person") => {
                new_channel.podcast_person.push(parse_podcast_person(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "location") => {
                new_channel
                    .podcast_location
                    .push(parse_podcast_location(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "trailer") => {
                new_channel
                    .podcast_trailer
                    .push(parse_podcast_trailer(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "license") => {
                new_channel
                    .podcast_license
                    .push(parse_podcast_license(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "liveItem") => {
                new_channel
                    .podcast_live_item
                    .push(parse_podcast_live_item(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "guid") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.podcast_guid.push(podcast::Guid::parse(&text));
                }
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "value") => {
                new_channel.podcast_value.push(parse_podcast_value(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "medium") => {
                if let Some(text) = parse_text_node(child) {
                    new_channel
                        .podcast_medium
                        .push(podcast::Medium::parse(&text));
                }
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "images") => {
                new_channel.podcast_images.push(parse_podcast_images(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "block") => {
                new_channel.podcast_block.push(parse_podcast_block(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "txt") => {
                new_channel.podcast_txt.push(parse_podcast_txt(child));
            }

            _ => {}
        }
    }

    new_channel
}

fn parse_item(item: roxmltree::Node) -> rss::Item {
    let mut new_item = rss::Item::default();

    for child in item.children() {
        let namespace = child.tag_name().namespace();
        let name = child.tag_name().name();
        match (namespace, name) {
            (None, "description") => {
                if let Some(text) = parse_text_node(child) {
                    new_item.description.push(text);
                }
            }
            (None, "link") => {
                if let Some(text) = parse_text_node(child) {
                    new_item
                        .link
                        .push(Url::parse(&text, UrlConstraint::HttpOrHttps));
                }
            }
            (None, "title") => {
                if let Some(text) = parse_text_node(child) {
                    new_item.title.push(text);
                }
            }
            (None, "enclosure") => {
                new_item.enclosure.push(parse_enclosure(child));
            }
            (None, "guid") => {
                new_item.guid.push(parse_guid(child));
            }
            (None, "pubDate") => {
                if let Some(text) = parse_text_node(child) {
                    new_item
                        .pub_date
                        .push(DateTime::parse(&text, TimeFormat::Rfc2822));
                }
            }

            (Some(NS_CONTENT), "encoded") => {
                if let Some(text) = parse_text_node(child) {
                    new_item.content_encoded.push(text);
                }
            }

            (Some(NS_ITUNES), "block") => {
                if let Some(text) = parse_text_node(child) {
                    new_item.itunes_block.push(itunes::Yes::parse(&text));
                }
            }
            (Some(NS_ITUNES), "duration") => {
                if let Some(text) = parse_text_node(child) {
                    new_item
                        .itunes_duration
                        .push(Number::parse(&text, NumberConstraint::NonNegative));
                }
            }
            (Some(NS_ITUNES), "season") => {
                if let Some(text) = parse_text_node(child) {
                    new_item
                        .itunes_season
                        .push(Integer::parse(&text, NumberConstraint::Positive));
                }
            }
            (Some(NS_ITUNES), "episode") => {
                if let Some(text) = parse_text_node(child) {
                    new_item
                        .itunes_episode
                        .push(Integer::parse(&text, NumberConstraint::Positive));
                }
            }
            (Some(NS_ITUNES), "explicit") => {
                if let Some(text) = parse_text_node(child) {
                    new_item
                        .itunes_explicit
                        .push(Bool::parse(&text, BoolType::TrueFalse));
                }
            }
            (Some(NS_ITUNES), "image") => {
                new_item.itunes_image.push(parse_itunes_image(child));
            }
            (Some(NS_ITUNES), "title") => {
                if let Some(text) = parse_text_node(child) {
                    new_item.itunes_title.push(text);
                }
            }
            (Some(NS_ITUNES), "episodeType") => {
                if let Some(text) = parse_text_node(child) {
                    new_item.itunes_type.push(itunes::EpisodeType::parse(&text));
                }
            }

            (Some(NS_PODCAST_1 | NS_PODCAST_2), "transcript") => {
                new_item
                    .podcast_transcript
                    .push(parse_podcast_transcript(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "chapters") => {
                new_item
                    .podcast_chapters
                    .push(parse_podcast_chapters(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "soundbite") => {
                new_item
                    .podcast_soundbite
                    .push(parse_podcast_soundbite(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "person") => {
                new_item.podcast_person.push(parse_podcast_person(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "location") => {
                new_item
                    .podcast_location
                    .push(parse_podcast_location(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "season") => {
                new_item.podcast_season.push(parse_podcast_season(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "episode") => {
                new_item.podcast_episode.push(parse_podcast_episode(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "alternateEnclosure") => {
                new_item
                    .podcast_alternate_enclosure
                    .push(parse_podcast_alternate_enclosure(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "value") => {
                new_item.podcast_value.push(parse_podcast_value(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "images") => {
                new_item.podcast_images.push(parse_podcast_images(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "socialInteract") => {
                new_item
                    .podcast_social_interact
                    .push(parse_podcast_social_interact(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "license") => {
                new_item.podcast_license.push(parse_podcast_license(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "txt") => {
                new_item.podcast_txt.push(parse_podcast_txt(child));
            }

            _ => {}
        }
    }

    new_item
}

fn parse_text_node(node: roxmltree::Node) -> Option<String> {
    if let Some(text) = node.text() {
        let text = text.trim();
        if !text.is_empty() {
            return Some(text.to_string());
        }
    }

    None
}

fn parse_enclosure(enclosure: roxmltree::Node) -> rss::Enclosure {
    let mut new_enclosure = rss::Enclosure::default();

    for attribute in enclosure.attributes() {
        match attribute.name() {
            "url" => {
                new_enclosure.url = Some(Url::parse(attribute.value(), UrlConstraint::HttpOrHttps));
            }
            "length" => {
                new_enclosure.length = Some(Integer::parse(
                    attribute.value(),
                    NumberConstraint::NonNegative,
                ));
            }
            "type" => {
                new_enclosure.type_ = Some(mime::Enclosure::parse(attribute.value()));
            }

            _ => {}
        }
    }

    new_enclosure
}

fn parse_guid(guid: roxmltree::Node) -> rss::Guid {
    let mut new_guid = rss::Guid::default();

    for attribute in guid.attributes() {
        if attribute.name() == "isPermaLink" {
            new_guid.is_permalink = Some(Bool::parse(attribute.value(), BoolType::TrueFalse));
        }
    }

    if let Some(text) = parse_text_node(guid) {
        new_guid.value = Some(rss::GuidValue::parse(&text, &new_guid.is_permalink));
    }

    new_guid
}

fn parse_itunes_category(category: roxmltree::Node) -> itunes::Category {
    let mut new_category = itunes::Category {
        ..Default::default()
    };

    for attribute in category.attributes() {
        if attribute.name() == "text" {
            new_category.text = Some(itunes::CategoryName::parse(attribute.value()));
        }
    }

    for child in category.children() {
        if child.tag_name().name() == "category" {
            new_category
                .subcategory
                .push(parse_itunes_subcategory(child));
        }
    }

    new_category
}

fn parse_itunes_subcategory(subcategory: roxmltree::Node) -> itunes::Subcategory {
    let mut new_subcategory = itunes::Subcategory {
        ..Default::default()
    };

    for attribute in subcategory.attributes() {
        if attribute.name() == "text" {
            new_subcategory.text = Some(itunes::SubcategoryName::parse(attribute.value()));
        }
    }

    new_subcategory
}

pub fn parse_itunes_image(image: roxmltree::Node) -> itunes::Image {
    let mut new_image = itunes::Image {
        ..Default::default()
    };

    for attribute in image.attributes() {
        if attribute.name() == "href" {
            new_image.href = Some(Url::parse(attribute.value(), UrlConstraint::HttpOrHttps));
        }
    }

    new_image
}

pub fn parse_itunes_owner(owner: roxmltree::Node) -> itunes::Owner {
    let mut new_owner = itunes::Owner {
        ..Default::default()
    };

    for child in owner.children() {
        match child.tag_name().name() {
            "email" => {
                if let Some(text) = parse_text_node(child) {
                    new_owner.email.push(text);
                }
            }
            "name" => {
                if let Some(text) = parse_text_node(child) {
                    new_owner.name.push(text);
                }
            }
            _ => {}
        }
    }

    new_owner
}

pub fn parse_podcast_locked(locked: roxmltree::Node) -> podcast::Locked {
    let mut new_locked = podcast::Locked {
        ..Default::default()
    };

    for attribute in locked.attributes() {
        if attribute.name() == "owner" {
            new_locked.owner = Some(attribute.value().to_string());
        }
    }

    if let Some(text) = parse_text_node(locked) {
        new_locked.value = Some(Bool::parse(&text, BoolType::YesNo));
    }

    new_locked
}

pub fn parse_podcast_funding(funding: roxmltree::Node) -> podcast::Funding {
    let mut new_funding = podcast::Funding {
        ..Default::default()
    };

    for attribute in funding.attributes() {
        if attribute.name() == "url" {
            new_funding.url = Some(Url::parse(attribute.value(), UrlConstraint::HttpsOnly));
        }
    }

    if let Some(text) = parse_text_node(funding) {
        new_funding.value = Some(text);
    }

    new_funding
}

pub fn parse_podcast_person(person: roxmltree::Node) -> podcast::Person {
    let mut new_person = podcast::Person {
        ..Default::default()
    };

    for attribute in person.attributes() {
        match attribute.name() {
            "role" => new_person.role = Some(podcast::PersonRole::parse(attribute.value())),
            "group" => new_person.group = Some(podcast::PersonGroup::parse(attribute.value())),
            "img" => new_person.img = Some(Url::parse(attribute.value(), UrlConstraint::HttpsOnly)),
            "href" => {
                new_person.href = Some(Url::parse(attribute.value(), UrlConstraint::HttpsOnly))
            }
            _ => {}
        }
    }

    if let Some(text) = parse_text_node(person) {
        new_person.value = Some(text);
    }

    new_person
}

pub fn parse_podcast_location(location: roxmltree::Node) -> podcast::Location {
    let mut new_location = podcast::Location {
        ..Default::default()
    };

    for attribute in location.attributes() {
        match attribute.name() {
            "geo" => new_location.geo = Some(podcast::Geo::parse(attribute.value())),
            "osm" => new_location.osm = Some(podcast::Osm::parse(attribute.value())),
            _ => {}
        }
    }

    if let Some(text) = parse_text_node(location) {
        new_location.value = Some(text);
    }

    new_location
}

pub fn parse_podcast_trailer(trailer: roxmltree::Node) -> podcast::Trailer {
    let mut new_trailer = podcast::Trailer {
        ..Default::default()
    };

    for attribute in trailer.attributes() {
        match attribute.name() {
            "url" => {
                new_trailer.url = Some(Url::parse(attribute.value(), UrlConstraint::HttpsOnly))
            }
            "pubdate" => {
                new_trailer.pub_date = Some(DateTime::parse(attribute.value(), TimeFormat::Rfc2822))
            }
            "length" => {
                new_trailer.length = Some(Integer::parse(
                    attribute.value(),
                    NumberConstraint::NonNegative,
                ))
            }
            "type" => new_trailer.type_ = Some(mime::Enclosure::parse(attribute.value())),
            "season" => {
                new_trailer.season = Some(Integer::parse(
                    attribute.value(),
                    NumberConstraint::NonNegative,
                ))
            }
            _ => {}
        }
    }

    if let Some(text) = parse_text_node(trailer) {
        new_trailer.value = Some(text);
    }

    new_trailer
}

fn parse_podcast_license(license: roxmltree::Node) -> podcast::License {
    let mut new_license = podcast::License {
        ..Default::default()
    };

    for attribute in license.attributes() {
        if attribute.name() == "url" {
            new_license.url = Some(Url::parse(attribute.value(), UrlConstraint::HttpsOnly));
        }
    }

    if let Some(text) = parse_text_node(license) {
        new_license.value = Some(podcast::LicenseType::parse(&text));
    }

    new_license
}

fn parse_podcast_value(value: roxmltree::Node) -> podcast::Value {
    let mut new_value = podcast::Value {
        ..Default::default()
    };

    for attribute in value.attributes() {
        match attribute.name() {
            "type" => new_value.type_ = Some(podcast::ValueType::parse(attribute.value())),
            "method" => new_value.method = Some(podcast::ValueMethod::parse(attribute.value())),
            "suggested" => {
                new_value.suggested =
                    Some(Float::parse(attribute.value(), NumberConstraint::Positive))
            }
            _ => {}
        }
    }

    for child in value.children() {
        let namespace = child.tag_name().namespace();
        let tag_name = child.tag_name().name();

        if let (Some(NS_PODCAST_1 | NS_PODCAST_2), "valueRecipient") = (namespace, tag_name) {
            new_value
                .value_recipient
                .push(parse_podcast_value_recipient(child));
        }
    }

    new_value
}

fn parse_podcast_value_recipient(value_recipient: roxmltree::Node) -> podcast::ValueRecipient {
    let mut new_value_recipient = podcast::ValueRecipient {
        ..Default::default()
    };

    for attribute in value_recipient.attributes() {
        match attribute.name() {
            "name" => new_value_recipient.name = Some(attribute.value().to_string()),
            "customKey" => new_value_recipient.custom_key = Some(attribute.value().to_string()),
            "customValue" => new_value_recipient.custom_value = Some(attribute.value().to_string()),
            "type" => {
                new_value_recipient.type_ =
                    Some(podcast::ValueRecipientType::parse(attribute.value()))
            }
            "address" => new_value_recipient.address = Some(attribute.value().to_string()),
            "split" => {
                new_value_recipient.split = Some(Integer::parse(
                    attribute.value(),
                    NumberConstraint::Positive,
                ))
            }
            "fee" => {
                new_value_recipient.fee = Some(Bool::parse(attribute.value(), BoolType::TrueFalse))
            }
            _ => {}
        }
    }

    new_value_recipient
}

pub fn parse_podcast_images(images: roxmltree::Node) -> podcast::Images {
    let mut new_images = podcast::Images {
        ..Default::default()
    };

    for attribute in images.attributes() {
        if attribute.name() == "srcset" {
            new_images.srcset = podcast::Images::parse_srcset(attribute.value());
        }
    }

    new_images
}

pub fn parse_podcast_block(block: roxmltree::Node) -> podcast::Block {
    let mut new_block = podcast::Block {
        ..Default::default()
    };

    for attribute in block.attributes() {
        if attribute.name() == "id" {
            new_block.id = Some(podcast::Service::parse(attribute.value()));
        }
    }

    if let Some(text) = parse_text_node(block) {
        new_block.value = Some(Bool::parse(&text, BoolType::YesNo));
    }

    new_block
}

pub fn parse_podcast_txt(txt: roxmltree::Node) -> podcast::Txt {
    let mut new_txt = podcast::Txt {
        ..Default::default()
    };

    for attribute in txt.attributes() {
        if attribute.name() == "purpose" {
            new_txt.purpose = Some(podcast::TxtPurpose::parse(attribute.value()));
        }
    }

    if let Some(text) = parse_text_node(txt) {
        new_txt.value = Some(text);
    }

    new_txt
}

pub fn parse_podcast_transcript(transcript: roxmltree::Node) -> podcast::Transcript {
    let mut new_transcript = podcast::Transcript {
        ..Default::default()
    };

    for attribute in transcript.attributes() {
        match attribute.name() {
            "url" => {
                new_transcript.url = Some(Url::parse(attribute.value(), UrlConstraint::HttpsOnly))
            }
            "type" => new_transcript.type_ = Some(mime::Transcript::parse(attribute.value())),
            "language" => new_transcript.language = Some(Language::parse(attribute.value())),
            "rel" => new_transcript.rel = Some(podcast::TranscriptRel::parse(attribute.value())),
            _ => {}
        }
    }

    new_transcript
}

pub fn parse_podcast_chapters(chapter: roxmltree::Node) -> podcast::Chapters {
    let mut new_chapter = podcast::Chapters {
        ..Default::default()
    };

    for attribute in chapter.attributes() {
        match attribute.name() {
            "url" => {
                new_chapter.url = Some(Url::parse(attribute.value(), UrlConstraint::HttpsOnly))
            }
            "type" => new_chapter.type_ = Some(mime::Chapters::parse(attribute.value())),
            _ => {}
        }
    }

    new_chapter
}

pub fn parse_podcast_soundbite(soundbite: roxmltree::Node) -> podcast::Soundbite {
    let mut new_soundbite = podcast::Soundbite {
        ..Default::default()
    };

    for attribute in soundbite.attributes() {
        match attribute.name() {
            "startTime" => {
                new_soundbite.start_time = Some(Float::parse(
                    attribute.value(),
                    NumberConstraint::NonNegative,
                ))
            }
            "duration" => {
                new_soundbite.duration =
                    Some(Float::parse(attribute.value(), NumberConstraint::Positive))
            }
            _ => {}
        }
    }

    if let Some(text) = parse_text_node(soundbite) {
        new_soundbite.value = Some(text);
    }

    new_soundbite
}

pub fn parse_podcast_season(season: roxmltree::Node) -> podcast::Season {
    let mut new_season = podcast::Season {
        ..Default::default()
    };

    for attribute in season.attributes() {
        if attribute.name() == "name" {
            new_season.name = Some(attribute.value().to_string());
        }
    }

    if let Some(text) = parse_text_node(season) {
        new_season.value = Some(Integer::parse(&text, NumberConstraint::NonNegative));
    }

    new_season
}

pub fn parse_podcast_episode(episode: roxmltree::Node) -> podcast::Episode {
    let mut new_episode = podcast::Episode {
        ..Default::default()
    };

    for attribute in episode.attributes() {
        if attribute.name() == "display" {
            new_episode.display = Some(attribute.value().to_string());
        }
    }

    if let Some(text) = parse_text_node(episode) {
        new_episode.value = Some(Number::parse(&text, NumberConstraint::NonNegative));
    }

    new_episode
}

pub fn parse_podcast_alternate_enclosure(
    alternate_enclosure: roxmltree::Node,
) -> podcast::AlternateEnclosure {
    let mut new_alternate_enclosure = podcast::AlternateEnclosure {
        ..Default::default()
    };

    for attribute in alternate_enclosure.attributes() {
        match attribute.name() {
            "type" => {
                new_alternate_enclosure.type_ = Some(mime::Enclosure::parse(attribute.value()))
            }
            "length" => {
                new_alternate_enclosure.length = Some(Integer::parse(
                    attribute.value(),
                    NumberConstraint::NonNegative,
                ))
            }
            "bitrate" => {
                new_alternate_enclosure.bit_rate = Some(Float::parse(
                    attribute.value(),
                    NumberConstraint::NonNegative,
                ))
            }
            "height" => {
                new_alternate_enclosure.height = Some(Integer::parse(
                    attribute.value(),
                    NumberConstraint::NonNegative,
                ))
            }
            "lang" => new_alternate_enclosure.language = Some(Language::parse(attribute.value())),
            "title" => new_alternate_enclosure.title = Some(attribute.value().to_string()),
            "rel" => new_alternate_enclosure.rel = Some(attribute.value().to_string()),
            "default" => {
                new_alternate_enclosure.default =
                    Some(Bool::parse(attribute.value(), BoolType::TrueFalse))
            }
            _ => {}
        }
    }

    for child in alternate_enclosure.children() {
        let namespace = child.tag_name().namespace();
        let name = child.tag_name().name();
        match (namespace, name) {
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "source") => new_alternate_enclosure
                .podcast_source
                .push(parse_podcast_source(child)),
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "integrity") => new_alternate_enclosure
                .podcast_integrity
                .push(parse_podcast_integrity(child)),
            _ => {}
        }
    }

    new_alternate_enclosure
}

pub fn parse_podcast_source(source: roxmltree::Node) -> podcast::Source {
    let mut new_source = podcast::Source {
        ..Default::default()
    };

    for attribute in source.attributes() {
        match attribute.name() {
            "contentType" => new_source.type_ = Some(mime::Enclosure::parse(attribute.value())),
            "uri" => {
                new_source.uri = Some(Url::parse(attribute.value(), UrlConstraint::AnyButHttp))
            }
            _ => {}
        }
    }

    new_source
}

pub fn parse_podcast_integrity(integrity: roxmltree::Node) -> podcast::Integrity {
    let mut new_integrity = podcast::Integrity {
        ..Default::default()
    };

    for attribute in integrity.attributes() {
        match attribute.name() {
            "type" => new_integrity.type_ = Some(podcast::IntegrityType::parse(attribute.value())),
            "value" => new_integrity.value = Some(attribute.value().to_string()),
            _ => {}
        }
    }

    new_integrity
}

pub fn parse_podcast_social_interact(social_interact: roxmltree::Node) -> podcast::SocialInteract {
    let mut new_social_interact = podcast::SocialInteract {
        ..Default::default()
    };

    for attribute in social_interact.attributes() {
        match attribute.name() {
            "uri" => {
                new_social_interact.uri =
                    Some(Url::parse(attribute.value(), UrlConstraint::HttpsOnly))
            }
            "protocol" => {
                new_social_interact.protocol =
                    Some(podcast::SocialProtocol::parse(attribute.value()))
            }
            "accountId" => new_social_interact.account_id = Some(attribute.value().to_string()),
            "accountUrl" => {
                new_social_interact.account_url =
                    Some(Url::parse(attribute.value(), UrlConstraint::HttpsOnly))
            }
            "priority" => {
                new_social_interact.priority =
                    Some(Integer::parse(attribute.value(), NumberConstraint::None))
            }
            _ => {}
        }
    }

    new_social_interact
}

fn parse_podcast_live_item(live_item: roxmltree::Node) -> podcast::LiveItem {
    let mut new_live_item = podcast::LiveItem::default();

    for attribute in live_item.attributes() {
        match attribute.name() {
            "status" => {
                new_live_item.status = Some(podcast::LiveItemStatus::parse(attribute.value()))
            }
            "start" => {
                new_live_item.start = Some(DateTime::parse(attribute.value(), TimeFormat::Iso8601))
            }
            "end" => {
                new_live_item.end = Some(DateTime::parse(attribute.value(), TimeFormat::Iso8601))
            }
            _ => {}
        }
    }

    for child in live_item.children() {
        let namespace = child.tag_name().namespace();
        let name = child.tag_name().name();
        match (namespace, name) {
            (None, "description") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item.description.push(text);
                }
            }
            (None, "link") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item
                        .link
                        .push(Url::parse(&text, UrlConstraint::HttpsOnly));
                }
            }
            (None, "title") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item.title.push(text);
                }
            }
            (None, "enclosure") => {
                new_live_item.enclosure.push(parse_enclosure(child));
            }
            (None, "guid") => {
                new_live_item.guid.push(parse_guid(child));
            }
            (None, "pubDate") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item
                        .pub_date
                        .push(DateTime::parse(&text, TimeFormat::Rfc2822));
                }
            }

            (Some(NS_CONTENT), "encoded") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item.content_encoded.push(text);
                }
            }

            (Some(NS_ITUNES), "block") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item.itunes_block.push(itunes::Yes::parse(&text));
                }
            }
            (Some(NS_ITUNES), "duration") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item
                        .itunes_duration
                        .push(Number::parse(&text, NumberConstraint::NonNegative));
                }
            }
            (Some(NS_ITUNES), "season") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item
                        .itunes_season
                        .push(Integer::parse(&text, NumberConstraint::Positive));
                }
            }
            (Some(NS_ITUNES), "episode") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item
                        .itunes_episode
                        .push(Integer::parse(&text, NumberConstraint::Positive));
                }
            }
            (Some(NS_ITUNES), "explicit") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item
                        .itunes_explicit
                        .push(Bool::parse(&text, BoolType::TrueFalse));
                }
            }
            (Some(NS_ITUNES), "image") => {
                new_live_item.itunes_image.push(parse_itunes_image(child));
            }
            (Some(NS_ITUNES), "title") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item.itunes_title.push(text);
                }
            }
            (Some(NS_ITUNES), "episodeType") => {
                if let Some(text) = parse_text_node(child) {
                    new_live_item
                        .itunes_type
                        .push(itunes::EpisodeType::parse(&text));
                }
            }

            (Some(NS_PODCAST_1 | NS_PODCAST_2), "transcript") => {
                new_live_item
                    .podcast_transcript
                    .push(parse_podcast_transcript(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "chapters") => {
                new_live_item
                    .podcast_chapters
                    .push(parse_podcast_chapters(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "soundbite") => {
                new_live_item
                    .podcast_soundbite
                    .push(parse_podcast_soundbite(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "person") => {
                new_live_item
                    .podcast_person
                    .push(parse_podcast_person(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "location") => {
                new_live_item
                    .podcast_location
                    .push(parse_podcast_location(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "season") => {
                new_live_item
                    .podcast_season
                    .push(parse_podcast_season(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "episode") => {
                new_live_item
                    .podcast_episode
                    .push(parse_podcast_episode(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "alternateEnclosure") => {
                new_live_item
                    .podcast_alternate_enclosure
                    .push(parse_podcast_alternate_enclosure(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "value") => {
                new_live_item.podcast_value.push(parse_podcast_value(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "images") => {
                new_live_item
                    .podcast_images
                    .push(parse_podcast_images(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "socialInteract") => {
                new_live_item
                    .podcast_social_interact
                    .push(parse_podcast_social_interact(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "license") => {
                new_live_item
                    .podcast_license
                    .push(parse_podcast_license(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "txt") => {
                new_live_item.podcast_txt.push(parse_podcast_txt(child));
            }
            (Some(NS_PODCAST_1 | NS_PODCAST_2), "contentLink") => {
                new_live_item
                    .podcast_content_link
                    .push(parse_podcast_content_link(child));
            }

            _ => {}
        }
    }

    new_live_item
}

pub fn parse_podcast_content_link(content_link: roxmltree::Node) -> podcast::ContentLink {
    let mut new_podcast_content_link = podcast::ContentLink::default();

    for attribute in content_link.attributes() {
        if attribute.name() == "href" {
            new_podcast_content_link.href =
                Some(Url::parse(attribute.value(), UrlConstraint::HttpsOnly));
        }
    }

    if let Some(text) = parse_text_node(content_link) {
        new_podcast_content_link.value = Some(text);
    }

    new_podcast_content_link
}
