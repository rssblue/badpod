use serde::{Deserialize, Deserializer};

#[derive(Debug, PartialEq)]
pub enum EpisodeNumber {
    // Will keeps as string because floating-point numbers may get rounded in strange ways.
    Number(String),
    Other(String),
}

pub fn option_episode_number<'de, D>(deserializer: D) -> Result<Option<EpisodeNumber>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    match s.parse::<f64>() {
        Ok(number) => {
            if number < 0.0 {
                return Ok(Some(EpisodeNumber::Other(s)));
            }
            Ok(Some(EpisodeNumber::Number(s)))
        }
        Err(_) => Ok(Some(EpisodeNumber::Other(s))),
    }
}
