# Changelog

All notable changes to the `dom_smoothie` crate will be documented in this file.

## [Unreleased]

### Added
- Implemented a CLI tool (`dom_smoothie_cli`) for demonstration purposes.
- Implemented `is_probably_readable` function. 
A quick-and-dirty way of figuring out if the contents of a given document are suitable for processing with `Readability`.
- Implemented `Readability::is_probably_readable`. This method calls the above function but uses its internal document (`dom_query::Document`).

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
