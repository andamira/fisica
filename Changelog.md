# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]


## [0.2.0] - 2023-08-24

### Added
- add features: `alloc`, `no-std`, `safest`, `unsafe`, `unsafest`, `nightly_docs`.
- add `devela` dependency.
- add `all` root module.
- add changelog.

### Removed
- remove `safe` from the default features.
- disable `no-std` checks for now.

### Changed
- remove direct `paste` dependency.
- update warnings allowances.
- update `glam` dependency.

### Fixed
- make it compile without the standard library.
- update readme and licenses.
- add missing headers.

## [0.1.1] - 2023-03-03

### Added
- add new SI unit prefixes: Ronna (10e27), Quetta (10e30), ronto (10e-27), quecto (10e-30).

### Fixes
- misc project cleanup, updates and fixes.

## [0.1.0] - 2021-12-14

First release


[unreleased]: https://github.com/andamira/fisica/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/andamira/fisica/releases/tag/v0.2.0
[0.1.1]: https://github.com/andamira/fisica/releases/tag/v0.1.1
[0.1.0]: https://github.com/andamira/fisica/releases/tag/v0.1.0

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
