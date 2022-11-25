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
badpod = "0.2.4"
```

### Deserializing

```rust
let rss = match badpod::from_str(feed_str) {
    Ok(rss) => rss,
    // Usually, this shouldn't happen, even if the feed doesn't conform
    // to the traditional schema.
    Err(_) => panic!("Something went terribly wrong."),
};
```

## Features

### Find whether tags or attributes are missing

We don't enforce almost any requirements on what tags *must* be included in the feed---that's a decision for you to make!
Almost every field of structs provided by `badpod` is either an [Option](std::option::Option) or a [Vec](std::vec::Vec)tor.
This allows to detect whether an XML tag or attribute is included in the feed that was processed.
```rust
match channel.podcast_value {
    Some(_) => println!("You support Value4Value! Awesome!"),
    None => println!("Have you considered receiving payment from listeners?"),
};
```

### Deserializing complicated tags

`badpod` converts complex data in text format to something that is easier to work with.
If that is not possible, we provide enums with variant `Other`, which is meant to represent data that could not be deserialized.
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
    podcast::Geo::Other(s) => {
        println!("Could not parse geographical coordinates from \"{s}\".")
    }
};
```

`Other` variant can be especially useful in enums with many variants.
If you don't get a match with `Other`, you know that deserialization yielded something that is reasonably expected.
```rust
match language {
    Language::English(region) => println!("A variant of English!"),
    Language::Lithuanian => println!("Lithuanian!"),
    Language::Other(s) => println!("Unexpected language code \"{s}\"."),
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
match channel.itunes_explicit {
    Some(is_explicit) => {
        match is_explicit {
            Bool::Ok(b) => {
                println!("is explicit? \"{b}\"")
            }
            Bool::Other(s) => println!("could not parse boolean value from \"{s}\""),
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
