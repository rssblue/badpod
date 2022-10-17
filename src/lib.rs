mod basic;
mod language;
pub mod mimetype;
mod time;

pub mod itunes;
pub mod podcast;
pub mod rss;

pub use crate::basic::{Bool, Float, Integer, Number};
pub use crate::language::Language;
pub use crate::time::DateTime;
pub use rss::{Channel, Enclosure, Feed, Item, GUID, RSS};

pub fn from_str(feed_str: &str) -> Result<Feed, String> {
    match xml_serde::from_str::<Feed>(feed_str) {
        Ok(feed) => Ok(feed),
        Err(e) => Err(e.to_string()),
    }
}
