use crate::basic::{Integer, NumberConstraint};
use crate::strings::{Url, UrlConstraint};
use crate::Other;

/// Allows specifying different image sizes at either the episode or channel level.
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Images {
    pub srcset: SrcSet,
}

/// `srcset` attribute in [Images](Images) object.
#[derive(Debug, PartialEq, Eq)]
pub enum SrcSet {
    Ok(Vec<(url::Url, i64)>),
    Other(Other),
}

impl Default for SrcSet {
    fn default() -> Self {
        Self::Ok(Vec::new())
    }
}

impl std::fmt::Display for SrcSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok(srcset) => {
                let srcsets = srcset
                    .iter()
                    .map(|(url, width)| display_single_image(url, *width))
                    .collect::<Vec<String>>();
                write!(f, "{}", srcsets.join("\n"))
            }
            Self::Other((s, reason)) => write!(f, "{s}: {reason}"),
        }
    }
}

fn parse_single_image(s: &str) -> Result<(url::Url, i64), String> {
    let img_str = s.trim();

    let parts: Vec<&str> = img_str.split(' ').collect();

    if parts.len() != 2 {
        return Err(
            "each image should consist of a URL followed by a space and the image width"
                .to_string(),
        );
    }

    let (url, mut width_str) = (parts[0], parts[1]);

    let url = match Url::parse(url, UrlConstraint::HttpOrHttps) {
        Url::Ok(url) => url,
        Url::Other((_, reason)) => return Err(format!("invalid URL ({reason})")),
    };

    if !width_str.ends_with('w') {
        return Err("string denoting image width should end with \"w\"".to_string());
    }

    // Remove last character.
    let mut chars = width_str.chars();
    chars.next_back();
    width_str = chars.as_str();

    match Integer::parse(width_str, NumberConstraint::Positive) {
        Integer::Ok(width) => Ok((url, width)),
        Integer::Other((_, reason)) => Err(format!("invalid width ({reason})")),
    }
}

fn display_single_image(url: &url::Url, width: i64) -> String {
    format!("{url} {width}w")
}

impl Images {
    pub fn parse_srcset(s: &str) -> SrcSet {
        let image_strs = s.split(',');
        let mut images = vec![];
        for (idx, image_str) in image_strs.enumerate() {
            match parse_single_image(image_str) {
                Ok(image) => images.push(image),
                Err(reason) => {
                    return SrcSet::Other((
                        s.to_string(),
                        format!("invalid image at index {idx}: {reason}"),
                    ))
                }
            }
        }

        SrcSet::Ok(images)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_srcset() {
        let strings = vec![
            "https://example.com/images/ep1/pci_avatar-massive.jpg 1500w,
            https://example.com/images/ep1/pci_avatar-middle.jpg 600w,
            https://example.com/images/ep1/pci_avatar-small.jpg 300w,
            https://example.com/images/ep1/pci_avatar-tiny.jpg 150w",
            "https://example.com/images/ep1/pci_avatar-massive.jpg 1500w,
            https://example.com/images/ep1/pci_avatar-middle.jpg 6o0w,
            https://example.com/images/ep1/pci_avatar-small.jpg 300w,
            https://example.com/images/ep1/pci_avatar-tiny.jpg 150w",
        ];
        let srcsets = vec![
            SrcSet::Ok(vec![
                (
                    url::Url::from_str("https://example.com/images/ep1/pci_avatar-massive.jpg")
                        .unwrap(),
                    1500,
                ),
                (
                    url::Url::from_str("https://example.com/images/ep1/pci_avatar-middle.jpg")
                        .unwrap(),
                    600,
                ),
                (
                    url::Url::from_str("https://example.com/images/ep1/pci_avatar-small.jpg")
                        .unwrap(),
                    300,
                ),
                (
                    url::Url::from_str("https://example.com/images/ep1/pci_avatar-tiny.jpg")
                        .unwrap(),
                    150,
                ),
            ]),
            SrcSet::Other((
                "https://example.com/images/ep1/pci_avatar-massive.jpg 1500w,
            https://example.com/images/ep1/pci_avatar-middle.jpg 6o0w,
            https://example.com/images/ep1/pci_avatar-small.jpg 300w,
            https://example.com/images/ep1/pci_avatar-tiny.jpg 150w"
                    .to_string(),
                "invalid image at index 1: invalid width (should be an integer)".to_string(),
            )),
        ];

        for (s, srcset) in strings.iter().zip(srcsets.iter()) {
            pretty_assertions::assert_eq!(Images::parse_srcset(s), *srcset);
        }
    }
}
