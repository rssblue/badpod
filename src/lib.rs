#![doc = include_str!("../README.md")]

mod basic;
mod language;
mod mime;
mod rss;
mod time;

pub mod itunes;
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
pub use rss::from_str;
pub use rss::{Channel, Enclosure, Guid, Item, Rss};
