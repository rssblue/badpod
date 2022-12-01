use std::str::FromStr;

/// Allows specifying different image sizes at either the episode or channel level.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Images {
    // #[serde(rename = "$attr:srcset", deserialize_with = "vec_image", default)]
    pub srcset: Vec<Image>,
}

impl std::fmt::Display for Images {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_list: Vec<String> = self.srcset.iter().map(|x| format!("{x}")).collect();
        write!(f, "{}", str_list.join(",\n            "))
    }
}

/// Individual image in [Images](Images) object.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Image {
    Ok(String, i64),
    Other(String),
}

impl std::str::FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let img_str = s.trim();

        let parts: Vec<&str> = img_str.split(' ').collect();

        if parts.len() != 2 {
            return Ok(Self::Other(img_str.to_string()));
        }

        let (url, mut width_str) = (parts[0], parts[1]);

        if !width_str.ends_with('w') {
            return Ok(Self::Other(img_str.to_string()));
        }

        // Remove last character.
        let mut chars = width_str.chars();
        chars.next_back();
        width_str = chars.as_str();

        if let Ok(width) = width_str.parse::<i64>() {
            if width <= 0 {
                return Ok(Self::Other(img_str.to_string()));
            }
            return Ok(Image::Ok(url.to_string(), width));
        }

        Ok(Self::Other(img_str.to_string()))
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(url, width) => {
                let s = format!("{url} {width}w");
                write!(f, "{s}")
            }
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

// fn vec_image<'de, D>(deserializer: D) -> Result<Vec<Image>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = match String::deserialize(deserializer) {
//         Ok(s) => s,
//         Err(e) => return Err(e),
//     };
//
//     let image_strs = s.split(',');
//     let mut images = vec![];
//     for image_str in image_strs {
//         match Image::from_str(image_str) {
//             Ok(image) => images.push(image),
//             Err(_) => images.push(Image::Other(image_str.trim().to_string())),
//         };
//     }
//
//     Ok(images)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_image() {
        let strings = vec![
            "https://example.com/images/ep1/pci_avatar-massive.jpg 1500w",
            "https://example.com/images/ep1/pci_avatar-middle.jpg 6o0w",
        ];
        let images = vec![
            Image::Ok(
                "https://example.com/images/ep1/pci_avatar-massive.jpg".to_string(),
                1500,
            ),
            Image::Other("https://example.com/images/ep1/pci_avatar-middle.jpg 6o0w".to_string()),
        ];

        for (s, image) in strings.iter().zip(images.iter()) {
            pretty_assertions::assert_eq!(Image::from_str(s), Ok(image.clone()));
            pretty_assertions::assert_eq!(format!("{}", image), *s);
        }
    }

    #[test]
    fn test_images() {
        let strings = vec![
            "https://example.com/images/ep1/pci_avatar-massive.jpg 1500w,
            https://example.com/images/ep1/pci_avatar-middle.jpg 600w,
            https://example.com/images/ep1/pci_avatar-small.jpg 300w,
            https://example.com/images/ep1/pci_avatar-tiny.jpg 150w",
        ];
        let images_lst = vec![Images {
            srcset: vec![
                Image::Ok(
                    "https://example.com/images/ep1/pci_avatar-massive.jpg".to_string(),
                    1500,
                ),
                Image::Ok(
                    "https://example.com/images/ep1/pci_avatar-middle.jpg".to_string(),
                    600,
                ),
                Image::Ok(
                    "https://example.com/images/ep1/pci_avatar-small.jpg".to_string(),
                    300,
                ),
                Image::Ok(
                    "https://example.com/images/ep1/pci_avatar-tiny.jpg".to_string(),
                    150,
                ),
            ],
        }];

        for (s, images) in strings.iter().zip(images_lst.iter()) {
            pretty_assertions::assert_eq!(format!("{}", images), *s);
        }
    }
}
