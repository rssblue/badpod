use serde::{Deserialize, Deserializer};

/// Allows specifying different image sizes at either the episode or channel level.
#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Images {
    #[serde(rename = "$attr:srcset", deserialize_with = "vec_image", default)]
    pub srcset: Vec<Image>,
}

/// Individual image in [Images](Images) object.
#[derive(Debug, PartialEq, Eq)]
pub enum Image {
    Ok(String, i64),
    Other(String),
}

fn vec_image<'de, D>(deserializer: D) -> Result<Vec<Image>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = match String::deserialize(deserializer) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let image_strs = s.split(',');
    let mut images = vec![];
    for mut image_str in image_strs {
        image_str = image_str.trim();
        match de_image(image_str) {
            Ok(image) => images.push(image),
            Err(_) => images.push(Image::Other(image_str.to_string())),
        };
    }

    Ok(images)
}

pub fn de_image(img_str: &str) -> Result<Image, String> {
    let img_str = img_str.trim();
    let parts: Vec<&str> = img_str.split(' ').collect();

    if parts.len() != 2 {
        return Err("expected two parts".to_string());
    }

    let (url, mut width_str) = (parts[0], parts[1]);

    if !width_str.ends_with('w') {
        return Err("width string should end with a \"w\"".to_string());
    }

    // Remove last character.
    let mut chars = width_str.chars();
    chars.next_back();
    width_str = chars.as_str();

    match width_str.parse::<i64>() {
        Ok(width) => {
            if width <= 0 {
                return Err("width should be positive".to_string());
            }
            Ok(Image::Ok(url.to_string(), width))
        }
        Err(_) => Err("width should be an integer".to_string()),
    }
}
