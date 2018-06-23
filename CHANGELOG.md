# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
### Changed
- `Reference::EntityRef` contains `&str` and not `StrSpan` now.
- Rename `Stream::try_consume_char_reference` into `try_consume_reference`.
  And it will return `Reference` and not `char` now.

## [0.5.0] - 2018-06-14
### Added
- `StreamError::InvalidChar`.
- `StreamError::InvalidSpace`.
- `StreamError::InvalidString`.

### Changed
- `Stream::consume_reference` will return only `InvalidReference` error from now.
- `Error::InvalidTokenWithCause` merged into `Error::InvalidToken`.
- `Stream::gen_error_pos_from` does not require `mut self` from now.
- `StreamError::InvalidChar` requires `Vec<u8>` and not `String` from now.
- `ErrorPos` uses `u32` and not `usize` from now.

### Removed
- `failure` dependency.
- `log` dependency.

## [0.4.1] - 2018-05-23
### Added
- An ability to parse an XML fragment.

## [0.4.0] - 2018-04-21
### Changed
- Relicense from MIT to MIT/Apache-2.0.

### Removed
- `FromSpan` trait.
- `from_str` and `from_span` methods are removed. Use the `From` trait instead.

## [0.3.0] - 2018-04-10
### Changed
- Use `failure` instead of `error-chain`.
- Minimum Rust version is 1.18.
- New error messages.
- `TokenType` is properly public now.

### Removed
- `ChainedError`

## [0.2.0] - 2018-03-11
### Added
- Qualified name parsing.

### Changed
- **Breaking**. `Token::ElementStart` and `Token::Attribute` contains prefix
  and local part of the qualified name now.

## [0.1.2] - 2018-02-12
### Added
- `Stream::skip_ascii_spaces`.
- Small performance optimizations.

## [0.1.1] - 2018-01-17
### Changed
- `log` 0.3 -> 0.4

[Unreleased]: https://github.com/RazrFalcon/xmlparser/compare/v0.5.0...HEAD
[0.4.1]: https://github.com/RazrFalcon/xmlparser/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/RazrFalcon/xmlparser/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/RazrFalcon/xmlparser/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/RazrFalcon/xmlparser/compare/v0.1.0...v0.1.1
