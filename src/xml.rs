use xmltree::{Element, XMLNode};

pub fn sort_tags(feed_str: &str) -> String {
    let mut feed = Element::parse(feed_str.as_bytes()).unwrap();

    // Comments need to be removed to ensure tags are properly sorted.
    remove_comments(&mut feed);

    sort_children(&mut feed);

    let mut buf = Vec::new();
    feed.write(&mut buf).unwrap();

    String::from_utf8(buf).unwrap()
}

fn sort_children(element: &mut Element) {
    let mut children = element.children.clone();
    children.sort_by(|a, b| match (a, b) {
        (XMLNode::Element(a), XMLNode::Element(b)) => {
            let a_name = {
                if let Some(prefix) = &a.prefix {
                    format!("{:?}:{}", a.name, prefix)
                } else {
                    a.name.clone()
                }
            };
            let b_name = {
                if let Some(prefix) = &b.prefix {
                    format!("{:?}:{}", b.name, prefix)
                } else {
                    b.name.clone()
                }
            };

            a_name.cmp(&b_name)
        }
        _ => std::cmp::Ordering::Equal,
    });
    element.children = children;

    // Sort recursively.
    for child in element.children.iter_mut() {
        if let XMLNode::Element(element) = child {
            sort_children(element);
        }
    }
}

fn remove_comments(element: &mut Element) {
    let mut remove_idxs = vec![];
    for (idx, child) in element.children.iter_mut().enumerate() {
        if let XMLNode::Comment(_) = child {
            remove_idxs.push(idx);
        } else if let XMLNode::Element(element) = child {
            remove_comments(element);
        }
    }

    // Remove comments recursively.
    for idx in remove_idxs.iter().rev() {
        element.children.remove(*idx);
    }
}
