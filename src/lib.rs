use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Feed {
    pub rss: RSS,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct RSS {
    #[serde(rename = "$attr:version")]
    pub version: Option<String>,

    pub channel: Option<Channel>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Default)]
pub struct Channel {
    pub copyright: Option<String>,
    pub description: Option<String>,
    pub generator: Option<String>,
    pub language: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_element_into_struct() {
        let feed = xml_serde::from_str::<super::Feed>(
                r#"
<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd" xmlns:podcast="https://podcastindex.org/namespace/1.0">
  <channel>
    <copyright>© Example Company</copyright>
    <description><![CDATA[<p><strong>Example HTML description</strong></p>]]></description>
    <link>https://example.com</link>
    <title>Example Podcast</title>
  </channel>
</rss>
            "#
            )
            .unwrap();

        assert_eq!(
            feed,
            super::Feed {
                rss: super::RSS {
                    version: Some("1.0".to_string()),
                    channel: Some(super::Channel {
                        copyright: Some("© Example Company".to_string()),
                        description: Some(
                            "<p><strong>Example HTML description</strong></p>".to_string()
                        ),
                        link: Some("https://example.com".to_string()),
                        title: Some("Example Podcast".to_string()),
                        ..Default::default()
                    }),
                }
            }
        );
    }
}
