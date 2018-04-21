# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/RazrFalcon/xmlparser/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/RazrFalcon/xmlparser/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/RazrFalcon/xmlparser/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/RazrFalcon/xmlparser/compare/0.1.2...0.2.0
[0.1.2]: https://github.com/RazrFalcon/xmlparser/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/RazrFalcon/xmlparser/compare/0.1.0...0.1.1
