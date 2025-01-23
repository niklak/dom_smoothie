# Changelog

All notable changes to the `dom_smoothie` crate will be documented in this file.

## [Unreleased]

### Changed
 - Changed the `Readability::grab_article` method implementation to retain only the best attempt among failed attempts, instead of keeping all of them until the exit.
 - Internal code optimizations aimed to reduce execution time.

## [0.4.0] - 2025-01-08

### Added
- Implemented a `serde` optional crate feature, enabling `serde::Serialize` and `serde::Deserialize` traits for `Article`, `Metadata`, and `Config` structures. 

### Changed
- Reduced the number of regex checks since they can be replaced with `contains` checks.
- Updated the dependencies.
- Internal code change: use `dom_query::Document::base_uri` to extract the base uri instead of `dom_query::Matcher`. 
- Updated the code (Byline extraction and JSON-LD parsing) to align with Mozilla's recent updates to the Readability library ([118f015](https://github.com/mozilla/readability/commit/118f01538e167218bd86ffd493bd3466aec4870a)).
- **Breaking:** Revised `Readability::is_probably_readable` method: it now uses `Config::readable_min_score` and `Config::readable_min_content_length` from the instance configuration instead of accepting arguments.

## [0.3.0] - 2025-01-08

### Added
- Implemented a CLI tool (`dom_smoothie_cli`) for demonstration purposes.
- Implemented `is_probably_readable` function. 
A quick-and-dirty way of figuring out if the contents of a given document are suitable for processing with `Readability`.
- Implemented `Readability::is_probably_readable`. This method calls the above function but uses its internal document (`dom_query::Document`).
- Implemented `Readability::with_document` method, which allows to create a new `Readability` instance with external `dom_query::Document`.

### Changed
- Changed visibility of `get_text_density`, `normalize_spaces`, and `link_density` to `pub(crate)` since they are used internally only.
- Refactor `Readability::parse_json_ld`.
- `Readability::parse_json_ld` also tries to extract `dateModified` and `image` from `ld+json` script. 

### Fixed
- `Article.text_content` accidentally contained text content of the original document. Now it contains only the text content of the article after processing.

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
