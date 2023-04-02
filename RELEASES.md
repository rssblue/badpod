# Releases

## Version 0.7.1 (2023-04-02)

- Simplify adding `badpod` to the project.

## Version 0.7.0 (2022-12-13)

- Improve iTunes' categories validation.
- Use Markdown in `Other` type.
- Other minor fixes and improvements.

## Version 0.6.0 (2022-12-04)

- Improve `<podcast:images>`.
- Add `<docs>` to `<channel>`.
- Add Apache license.

## Version 0.5.0 (2022-12-03)

- Provide reason when enum deserialization fails.
- Introduce `Url` enum type.

## Version 0.4.0 (2022-12-01)

- Use `roxmltree` instead of `serde` for parsing; this improves performance by around 2X and also avoids some deserialization errors.
- Other minor fixes and improvements.

## Version 0.3.2 (2022-11-28)

- Improve parsing of ISO 8601 dates.

## Version 0.3.1 (2022-11-28)

- Make fields between `<item>` and `<podcast:liveItem>` consistent.

## Version 0.3.0 (2022-11-27)

- Allow multiple tags for everything.

## Version 0.2.4 (2022-11-25)

- Improve tests.
- Handle multiple non-neighboring tags with the same name.

## Version 0.2.3 (2022-11-16)

- Remove `Eq` from floats.

## Version 0.2.2 (2022-10-30)

- Add support for `<podcast:txt>`.
- Implement `Display` trait for all enums.
- Other minor fixes and improvements.

## Version 0.2.1 (2022-10-28)

- Fix version in README.

## Version 0.2.0 (2022-10-28)

- Ensure enums can be displayed without panicking.
- Implement proper displaying of languages with regions.
- Add `<category>`, `<lastBuildDate>`, `<managingEditor>`, `<pubDate>`, `<ttl>`, and `<webMaster>` to channel level.
- Other minor fixes and improvements.
