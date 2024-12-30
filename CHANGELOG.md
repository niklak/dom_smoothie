# Changelog

All notable changes to the `dom_smoothie` crate will be documented in this file.

## [0.2.0] - 2024-12-30

### Added

- Implement support for `Config::max_elements_to_parse` in `Readability::parse`.
- Implement support for `Config::disable_json_ld` in `Readability::parse`.
- Implement support for `Config::n_top_candidates` in `Readability::parse`.
- Implement support for `Config::char_threshold` in `Readability::parse`.


### Fixed

- Improve parsing of article's metadata (title, byline, excerpt, site_name, published_time, lang, and dir).
- Improve parsing of `dir` attribute.
- Fix the internal behavior of `Readability::clean_classes` when `Config::classes_to_preserve` is empty.


## [0.1.1] - 2024-12-18

### Changed

- Changed visibility of the `Readability::prepare` to private.
- The `Article` struct is now visible.

### Added

- Added documentation for public API.
