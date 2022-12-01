#![doc = include_str!("../README.md")]

mod basic;
mod language;
mod mime;
mod parse;
mod rss;
mod time;
mod utils;

/// Namespace extension for Apple Podcasts.
///
/// Implementation details adapted from
/// <https://help.apple.com/itc/podcasts_connect/#/itcb54353390>.
pub mod itunes;

/// Namespace extension for podcasting.
///
/// Implementation details adapted from <https://podcastindex.org/namespace/1.0>.
pub mod podcast;

pub use crate::basic::{Bool, Float, Integer, Number};
pub use crate::language::{
    Language, LanguageChinese, LanguageDutch, LanguageEnglish, LanguageFrench, LanguageGerman,
    LanguageItalian, LanguagePortugese, LanguageRomanian, LanguageRussian, LanguageSpanish,
    LanguageSwedish,
};
pub use crate::mime::{
    Chapters as MimeChapters, Enclosure as MimeEnclosure, Transcript as MimeTranscript,
};
pub use crate::time::DateTime;
pub use parse::{from_str, Error};
pub use rss::{Channel, Enclosure, Guid, Item, Rss};
