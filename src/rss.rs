use crate::itunes;
use crate::podcast;

use crate::basic;
use crate::language;
use crate::mime;
use crate::time;

#[derive(Debug, PartialEq, Default)]
pub struct Xml {
    pub rss: Rss,
}

/// RSS feed.
#[derive(Debug, PartialEq, Default)]
pub struct Rss {
    pub version: Option<String>,
    pub channel: Vec<Channel>,
}

/// Podcast feed.
#[derive(Debug, PartialEq, Default)]
pub struct Channel {
    pub category: Vec<String>,
    pub copyright: Vec<String>,
    pub description: Vec<String>,
    pub generator: Vec<String>,
    pub item: Vec<Item>,
    pub language: Vec<language::Language>,
    pub last_build_date: Vec<time::DateTime>,
    pub link: Vec<String>,
    pub managing_editor: Vec<String>,
    pub pub_date: Vec<time::DateTime>,
    pub title: Vec<String>,
    pub ttl: Vec<basic::Integer>,
    pub web_master: Vec<String>,

    pub content_encoded: Vec<String>,

    pub itunes_author: Vec<String>,
    pub itunes_block: Vec<itunes::Yes>,
    pub itunes_category: Vec<itunes::Category>,
    pub itunes_complete: Vec<itunes::Yes>,
    pub itunes_explicit: Vec<basic::Bool>,
    pub itunes_image: Vec<itunes::Image>,
    pub itunes_new_feed_url: Vec<String>,
    pub itunes_owner: Vec<itunes::Owner>,
    pub itunes_type: Vec<itunes::PodcastType>,

    pub podcast_locked: Vec<podcast::Locked>,
    pub podcast_funding: Vec<podcast::Funding>,
    pub podcast_person: Vec<podcast::Person>,
    pub podcast_location: Vec<podcast::Location>,
    pub podcast_trailer: Vec<podcast::Trailer>,
    pub podcast_license: Vec<podcast::License>,
    pub podcast_guid: Vec<podcast::Guid>,
    pub podcast_value: Vec<podcast::Value>,
    pub podcast_medium: Vec<podcast::Medium>,
    pub podcast_images: Vec<podcast::Images>,
    pub podcast_live_item: Vec<podcast::LiveItem>,
    pub podcast_block: Vec<podcast::Block>,
    pub podcast_txt: Vec<podcast::Txt>,
}

/// Podcast episode.
#[derive(Debug, PartialEq, Default)]
pub struct Item {
    pub description: Vec<String>,
    pub link: Vec<String>,
    pub title: Vec<String>,
    pub enclosure: Vec<Enclosure>,
    pub guid: Vec<Guid>,
    pub pub_date: Vec<time::DateTime>,

    pub content_encoded: Vec<String>,

    pub itunes_block: Vec<itunes::Yes>,
    pub itunes_duration: Vec<basic::Number>,
    pub itunes_season: Vec<basic::Integer>,
    pub itunes_episode: Vec<basic::Integer>,
    pub itunes_explicit: Vec<basic::Bool>,
    pub itunes_image: Vec<itunes::Image>,
    pub itunes_title: Vec<String>,
    pub itunes_type: Vec<itunes::EpisodeType>,

    pub podcast_transcript: Vec<podcast::Transcript>,
    pub podcast_chapters: Vec<podcast::Chapters>,
    pub podcast_soundbite: Vec<podcast::Soundbite>,
    pub podcast_person: Vec<podcast::Person>,
    pub podcast_location: Vec<podcast::Location>,
    pub podcast_season: Vec<podcast::Season>,
    pub podcast_episode: Vec<podcast::Episode>,
    pub podcast_alternate_enclosure: Vec<podcast::AlternateEnclosure>,
    pub podcast_value: Vec<podcast::Value>,
    pub podcast_images: Vec<podcast::Images>,
    pub podcast_social_interact: Vec<podcast::SocialInteract>,
    pub podcast_license: Vec<podcast::License>,
    pub podcast_txt: Vec<podcast::Txt>,
}

/// Episode's media content.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Enclosure {
    pub url: Option<String>,
    pub length: Option<basic::Integer>,
    pub type_: Option<mime::Enclosure>,
}

/// Episode's globally unique identifier.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Guid {
    pub is_permalink: Option<basic::Bool>,
    pub value: Option<String>,
}
