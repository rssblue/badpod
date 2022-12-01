use crate::rss;
use crate::time::TimeFormat;

#[derive(Debug, PartialEq)]
pub enum Error {
    NoRoot,
    RootNotRss,
    Custom(String),
}

pub fn parse(feed_str: &str) -> Result<rss::Rss, Error> {
    let tree = match roxmltree::Document::parse(feed_str) {
        Ok(tree) => tree,
        Err(roxmltree::Error::NoRootNode) => return Err(Error::NoRoot),
        Err(e) => return Err(Error::Custom(e.to_string())),
    };

    println!("{:?}", tree);

    let root = tree.root_element();
    if root.tag_name().name() != "rss" {
        return Err(Error::RootNotRss);
    }

    let mut feed = rss::Rss {
        ..Default::default()
    };
    for attribute in root.attributes() {
        if attribute.name() == "version" {
            feed.version = Some(attribute.value().to_string());
        }
    }
    for child in root.children() {
        if child.tag_name().name() == "channel" {
            feed.channel.push(parse_channel(child));
        }
    }

    Ok(feed)
}

fn parse_channel(channel: roxmltree::Node) -> rss::Channel {
    let mut new_channel = rss::Channel {
        ..Default::default()
    };

    for child in channel.children() {
        match child.tag_name().name() {
            "category" => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.category.push(text);
                }
            }
            "copyright" => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.copyright.push(text);
                }
            }
            "description" => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.description.push(text);
                }
            }
            "generator" => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.generator.push(text);
                }
            }
            "lastBuildDate" => {
                if let Some(text) = parse_text_node(child) {
                    new_channel
                        .last_build_date
                        .push(TimeFormat::Rfc2822.parse(&text));
                }
            }
            "title" => {
                if let Some(text) = parse_text_node(child) {
                    new_channel.title.push(text);
                }
            }
            _ => {}
        }
    }

    new_channel
}

fn parse_text_node(node: roxmltree::Node) -> Option<String> {
    if let Some(text) = node.text() {
        let text = text.trim();
        if !text.is_empty() {
            return Some(text.to_string());
        }
    }

    None
}
