use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct RSS {
    #[serde(rename = "$attr:version")]
    pub version: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Feed {
    #[serde(rename = "rss")]
    pub rss: RSS,
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_element_into_struct() {
        assert_eq!(
            xml_serde::from_str::<super::Feed>(
                r#"
<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd" xmlns:podcast="https://podcastindex.org/namespace/1.0">
</rss>
            "#
            )
            .unwrap(),
            super::Feed {
                rss: super::RSS {
                    version: Some("2.0".to_string()),
                }
            }
            );
    }
}
