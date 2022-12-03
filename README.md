# badpod

A Rust crate for working with imperfect feeds of podcasts.

This crate *should not* be used for  
❌ working with proper, processed podcast feeds  
❌ interfacing with your database

This crate *can be* used for  
✅ interpreting external feeds, often from unknown sources  
✅ providing feedback about the contents of the feed

## Motivation

On a backend server, which has to communicate with a database, strict schemas are typically used.
Therefore, if content from an external source that does not conform to the schema is loaded, one may fail to deserialize that content successfully.

This scenario is very common in podcasting space, where RSS feeds of podcasts may  
- combine multiple standards (namespaces)
- have some elements missing
- have some values in wrong data types
- etc.

In that case, we may want a more flexible, intermediate schema: a schema that does *not* throw an error the instant it encounters an unexpected value but rather one that **deserializes the content that it is able to** and **stores the content that it failed to deserialize** for further cleanup or analysis.

## Usage

### Including in a project

Inside `Cargo.toml`:  
```toml
[dependencies]
badpod = "0.5.0"
```

### Deserializing

```rust
let rss = match badpod::from_str(feed_str) {
    Ok(rss) => rss,
    Err(_) => panic!("Something went terribly wrong."),
};
```

In theory, `badpod::from_str` should only return an error in two cases:  
- the feed is not valid XML
- the root element of the feed is not `<rss>`

## Features

### Check for presence of tags and attributes

In `badpod`, every field representing an XML tag is a [Vec](std::vec::Vec)tor, and every field representing an XML attribute is an [Option](std::option::Option).
This is to reflect that in XML, tags *can* be repeated, while attributes---*cannot*.
We don't enforce any requirements on what or how many tags should be in the feed---that's a decision for you to make!
Leaving it more flexible (instead of throwing an instant error) also allows providing users with better feedback.
```rust
match my_channel.podcast_value.len() {
    0 => println!("Have you considered receiving payments from listeners?"),
    1 => println!("You support Value4Value! Awesome!"),
    _ => println!("Only one <podcast:value> tag is allowed per channel."),
}
```

*Note*: all fields representing tags are named in the singular form, even though they are vectors.

### Deserializing complicated tags

`badpod` converts complex data in text format to something that is easier to work with.
If that is not possible, we provide enums with variant `Other`, which is meant to represent data that could not be deserialized and the reason for that failed deserialization.
```rust
match geo {
    podcast::Geo::Ok {
        // f64
        latitude,
        // f64
        longitude,
        // Option<f64>
        altitude,
        // Option<f64>
        uncertainty,
    } => {
        println!("Successfully extracted geographical coordinates!")
    }
    podcast::Geo::Other((s, reason)) => {
        println!("Could not parse coordinates from \"{s}\": {reason}.")
    }
};
```

`Other` variant can be especially useful in enums with many variants.
If you don't get a match with `Other`, you know that deserialization yielded something that is reasonably expected.
```rust
match language {
    Language::English(region) => println!("A variant of English!"),
    Language::Lithuanian => println!("Lithuanian!"),
    Language::Other((s, _)) => println!("Unexpected language code \"{s}\"."),
    _ => println!("Some other valid language!"),
};
```

But just because you match a variant that is not `Other` does not mean that it is a valid value for *you*.
For example, [MimeEnclosure](crate::MimeEnclosure) has `AudioOpus` as one of the variants, but if you require that feeds only contain media files with formats supported by Apple Podcasts, then you will want to reject this.
Again, `badpod` is only a tool for analyzing feeds; you decide what a "proper" feed must look like.

### Tag-aware deserialization

Many tags use the same data types but encode them differently.
For example, both `<podcast:locked>` and `<itunes:explicit>` are essentially boolean values, but the former serializes to `"yes"`/`"no"` and the latter to `"true"`/`"false"`.
In `badpod`, both are deserialized to [Bool](crate::Bool).
```rust
match channel.itunes_explicit.get(0) {
    Some(is_explicit) => {
        match is_explicit {
            Bool::Ok(b) => {
                println!("is explicit? \"{b}\"")
            }
            Bool::Other((s, reason)) => println!("could not parse \"{s}\": {reason}"),
        }
    }
    None => println!("<itunes:explicit> not found."),
};
```

### Printing enums

Although this crate is mainly for deserialization, there are cases when enums need to be converted back to strings.
In `badpod`, all enums have [Display](std::fmt::Display) trait implemented:
```rust
// Outputs "cs".
println!("{}", Language::Czech);

// Outputs "en".
println!("{}", Language::English(LanguageEnglish::Default));

// Outputs "en-gb".
println!("{}", Language::English(LanguageEnglish::UnitedKingdom));
```
