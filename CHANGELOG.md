# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]
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

[Unreleased]: https://github.com/RazrFalcon/libxmlparser/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/RazrFalcon/libxmlparser/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/RazrFalcon/libxmlparser/compare/0.1.0...0.1.1
