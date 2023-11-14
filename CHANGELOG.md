# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
### Added
* Add support for using [`alloc::vec::Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html)
### Changed
* Due to dependency updates the MSRV has been updated from 1.60 to 1.62. This should only be relevant if you use the `defmt` feature, but we now only test with 1.62 and not older releases, so it's not guaranteed to work otherwise.
* Update to `heapless:0.8.0`
### Breaking Changes
* You must now select either the feature `use_alloc` or `use_heapless` for the crate to compile. Select `use_heapless`
  to keep the API from the previous release of this crate.

## [0.1.1] - 2023-01-07
### Changed
* The Minimum Supported Rust Version (MSRV) has been defined as 1.60.0 and is being enforced in `Cargo.toml`

<!-- next-url -->
[Unreleased]: https://github.com/rursprung/adafruit-bluefruit-protocol-rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/rursprung/adafruit-bluefruit-protocol-rs/compare/v0.1.0...v0.1.1
