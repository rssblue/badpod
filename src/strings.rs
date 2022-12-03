use crate::Other;
use url as url_ext;

pub enum UrlConstraint {
    AnyProtocol,
    HttpOrHttps,
    HttpsOnly,
    AnyButHttp,
}

/// Used for deserializing URLs.
#[derive(Debug, PartialEq, Eq)]
pub enum Url {
    Ok(url::Url),
    Other(Other),
}

impl Url {
    pub fn parse(s: &str, constraint: UrlConstraint) -> Self {
        let url = url_ext::Url::parse(s);
        match url {
            Ok(url) => match constraint {
                UrlConstraint::AnyProtocol => Self::Ok(url),
                UrlConstraint::HttpOrHttps => {
                    if url.scheme() != "http" && url.scheme() != "https" {
                        return Self::Other((
                            s.to_string(),
                            "protocol must be http or https".to_string(),
                        ));
                    }
                    Self::Ok(url)
                }
                UrlConstraint::HttpsOnly => {
                    if url.scheme() != "https" {
                        return Self::Other((s.to_string(), "protocol must be https".to_string()));
                    }
                    Self::Ok(url)
                }
                UrlConstraint::AnyButHttp => {
                    if url.scheme() == "http" {
                        return Self::Other((
                            s.to_string(),
                            "protocol must not be http".to_string(),
                        ));
                    }
                    Self::Ok(url)
                }
            },
            Err(_) => Url::Other((s.to_string(), "invalid URL".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Url::parse("https://example.com", UrlConstraint::AnyProtocol),
            Url::Ok(url::Url::parse("https://example.com").unwrap())
        );
        assert_eq!(
            Url::parse("https://example.com", UrlConstraint::HttpOrHttps),
            Url::Ok(url::Url::parse("https://example.com").unwrap())
        );
        assert_eq!(
            Url::parse("https://example.com", UrlConstraint::HttpsOnly),
            Url::Ok(url::Url::parse("https://example.com").unwrap())
        );
        assert_eq!(
            Url::parse("https://example.com", UrlConstraint::AnyButHttp),
            Url::Ok(url::Url::parse("https://example.com").unwrap())
        );
        assert_eq!(
            Url::parse("http://example.com", UrlConstraint::AnyProtocol),
            Url::Ok(url::Url::parse("http://example.com").unwrap())
        );
        assert_eq!(
            Url::parse("http://example.com", UrlConstraint::HttpOrHttps),
            Url::Ok(url::Url::parse("http://example.com").unwrap())
        );
        assert_eq!(
            Url::parse("http://example.com", UrlConstraint::HttpsOnly),
            Url::Other((
                "http://example.com".to_string(),
                "protocol must be https".to_string()
            ))
        );
        assert_eq!(
            Url::parse("http://example.com", UrlConstraint::AnyButHttp),
            Url::Other((
                "http://example.com".to_string(),
                "protocol must not be http".to_string()
            ))
        );
        assert_eq!(
            Url::parse("ftp://example.com", UrlConstraint::AnyProtocol),
            Url::Ok(url::Url::parse("ftp://example.com").unwrap())
        );
        assert_eq!(
            Url::parse("ftp://example.com", UrlConstraint::HttpOrHttps),
            Url::Other((
                "ftp://example.com".to_string(),
                "protocol must be http or https".to_string()
            ))
        );
        assert_eq!(
            Url::parse("ftp://example.com", UrlConstraint::HttpsOnly),
            Url::Other((
                "ftp://example.com".to_string(),
                "protocol must be https".to_string()
            ))
        );
        assert_eq!(
            Url::parse("example.com", UrlConstraint::AnyProtocol),
            Url::Other(("example.com".to_string(), "invalid URL".to_string()))
        );
        assert_eq!(
            Url::parse("http://", UrlConstraint::AnyProtocol),
            Url::Other(("http://".to_string(), "invalid URL".to_string()))
        );
        assert_eq!(
            Url::parse("http://example.com:abc", UrlConstraint::AnyProtocol),
            Url::Other((
                "http://example.com:abc".to_string(),
                "invalid URL".to_string()
            ))
        );
        assert_eq!(
            Url::parse("http://example.com:123", UrlConstraint::AnyProtocol),
            Url::Ok(url::Url::parse("http://example.com:123").unwrap())
        );
    }
}
