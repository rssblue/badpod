mod basic;
mod language;
pub mod mimetype;
mod rss;
mod time;

pub mod itunes;
pub mod podcast;

pub use crate::basic::{Bool, Float, Integer, Number};
pub use crate::language::Language;
pub use crate::time::DateTime;
pub use rss::{Channel, Enclosure, Item, Guid, Rss};

pub fn from_str(feed_str: &str) -> Result<Rss, String> {
    match xml_serde::from_str::<rss::Xml>(feed_str) {
        Ok(feed) => Ok(feed.rss),
        Err(e) => Err(e.to_string()),
    }
}
