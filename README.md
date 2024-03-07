# xmlparser

[<img alt="github" src="https://img.shields.io/badge/github-RazrFalcon/xmlparser-8da0cb?style=for-the-badge&logo=github" height="20">](https://github.com/RazrFalcon/xmlparser)
[<img alt="crates.io" src="https://img.shields.io/crates/v/xmlparser.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/xmlparser)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-xmlparser-66c2a5?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/xmlparser)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/RazrFalcon/xmlparser/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/RazrFalcon/xmlparser/actions?query=branch%3Amaster)

*xmlparser* is a low-level, pull-based, zero-allocation
[XML 1.0](https://www.w3.org/TR/xml/) parser.

<br>

## Example

```rust
for token in xmlparser::Tokenizer::from("<tagname name='value'/>") {
    println!("{:?}", token);
}
```

<br>

## Why a new library?

This library is basically a low-level XML tokenizer that preserves the
positions of the tokens and is not intended to be used directly.

If you are looking for a higher level solution, check out
[roxmltree](https://github.com/RazrFalcon/roxmltree).

<br>

## Benefits

- All tokens contain `StrSpan` structs which represent the position of the
  substring in the original document.
- Good error processing. All error types contain the position (line:column)
  where it occurred.
- No heap allocations.
- No dependencies.
- Tiny. ~1400 LOC and ~30KiB in the release build according to
  `cargo-bloat`.
- Supports `no_std` builds. To use without the standard library, disable the
  default features.

<br>

## Limitations

- Currently, only ENTITY objects are parsed from the DOCTYPE. All others are
  ignored.
- No tree structure validation. So an XML like
  `<root><child></root></child>` or a string without root element will be
  parsed without errors. You should check for this manually. On the other
  hand `<a/><a/>` will lead to an error.
- Duplicated attributes is not an error. So XML like `<item a="v1" a="v2"/>`
  will be parsed without errors. You should check for this manually.
- UTF-8 only.

<br>

## Safety

- The library must not panic. Any panic is considered a critical bug and
  should be reported.
- The library forbids unsafe code.

<br>

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE] or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

at your option.

<br>

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[LICENSE-APACHE]: https://github.com/RazrFalcon/xmlparser/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/RazrFalcon/xmlparser/blob/master/LICENSE-MIT
