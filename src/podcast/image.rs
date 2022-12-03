use crate::strings::{Url, UrlConstraint};
use crate::Other;
use std::str::FromStr;

/// Allows specifying different image sizes at either the episode or channel level.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Images {
    pub srcset: Vec<Image>,
}

impl std::fmt::Display for Images {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_list: Vec<String> = self.srcset.iter().map(|x| format!("{x}")).collect();
        write!(f, "{}", str_list.join(",\n            "))
    }
}

/// Individual image in [Images](Images) object.
#[derive(Debug, PartialEq, Eq)]
pub enum Image {
    Ok(url::Url, i64),
    Other(Other),
}

impl std::str::FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let img_str = s.trim();

        let parts: Vec<&str> = img_str.split(' ').collect();

        if parts.len() != 2 {
            return Ok(Self::Other((
                img_str.to_string(),
                "each image should consist of a URL followed by a space and the image width"
                    .to_string(),
            )));
        }

        let (url, mut width_str) = (parts[0], parts[1]);

        let url = match Url::parse(url, UrlConstraint::HttpOrHttps) {
            Url::Ok(url) => url,
            Url::Other(other) => return Ok(Self::Other(other)),
        };

        if !width_str.ends_with('w') {
            return Ok(Self::Other((
                img_str.to_string(),
                "string denoting image width should end with 'w'".to_string(),
            )));
        }

        // Remove last character.
        let mut chars = width_str.chars();
        chars.next_back();
        width_str = chars.as_str();

        if let Ok(width) = width_str.parse::<i64>() {
            if width <= 0 {
                return Ok(Self::Other((
                    img_str.to_string(),
                    "image width should be positive".to_string(),
                )));
            }
            return Ok(Image::Ok(url, width));
        }

        Ok(Self::Other((
            img_str.to_string(),
            "image width should be an integer".to_string(),
        )))
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(url, width) => {
                let s = format!("{url} {width}w");
                write!(f, "{s}")
            }
            Self::Other((s, _)) => write!(f, "{s}"),
        }
    }
}

impl Images {
    pub fn parse_srcset(s: &str) -> Vec<Image> {
        let image_strs = s.split(',');
        let mut images = vec![];
        for image_str in image_strs {
            match Image::from_str(image_str) {
                Ok(image) => images.push(image),
                Err(e) => images.push(Image::Other((image_str.trim().to_string(), e))),
            };
        }

        images
    }
}

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
                url::Url::parse("https://example.com/images/ep1/pci_avatar-massive.jpg").unwrap(),
                1500,
            ),
            Image::Other((
                "https://example.com/images/ep1/pci_avatar-middle.jpg 6o0w".to_string(),
                "image width should be an integer".to_string(),
            )),
        ];

        for (s, image) in strings.iter().zip(images.iter()) {
            pretty_assertions::assert_eq!(Image::from_str(s).unwrap(), *image);
            pretty_assertions::assert_eq!(format!("{}", &image), *s);
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
                    url::Url::parse("https://example.com/images/ep1/pci_avatar-massive.jpg")
                        .unwrap(),
                    1500,
                ),
                Image::Ok(
                    url::Url::parse("https://example.com/images/ep1/pci_avatar-middle.jpg")
                        .unwrap(),
                    600,
                ),
                Image::Ok(
                    url::Url::parse("https://example.com/images/ep1/pci_avatar-small.jpg").unwrap(),
                    300,
                ),
                Image::Ok(
                    url::Url::parse("https://example.com/images/ep1/pci_avatar-tiny.jpg").unwrap(),
                    150,
                ),
            ],
        }];

        for (s, images) in strings.iter().zip(images_lst.iter()) {
            pretty_assertions::assert_eq!(format!("{}", images), *s);
        }
    }
}
