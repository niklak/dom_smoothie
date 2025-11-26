# Changelog

All notable changes to the `dom_smoothie` crate will be documented in this file.

## [Unreleased]

### Added
- Implemented internal `AsciiPatternCheck`, improving the performance of `match_unlikely` and `determine_attr_weight`
 when the `aho-corasick` feature is disabled. No public API changes.

### Changed
- Updated `dom_query` version from `0.23.0` to `0.24.0`.
- Moved all secondary crates into `crates/`.
- Minor internal code changes.

## [0.13.0] - 2025-10-02
### Added
- Added `Metadata.favicon` and `Article.favicon` support when calling `Readability::get_article_metadata` and `Readability::parse`.
- Added *experimental* crate `dom-smoothie-lua` providing **Lua** bindings for the `dom_smoothie` crate.

### Changed
- Updated `dom_query` version from `0.22.0` to `0.23.0`.
- Revised `grab::score_elements`: use a cache for normalized char count to improve performance. No public API changes.
- Minor internal code changes.

### Fixed
- `MATCHER_LAZY_IMG` is now used in `prep_article::fix_lazy_images` instead of `MINI_LAZY`, since the latter does not support complex selectors.

## [0.12.0] - 2025-09-04

### Changed
- Optimized internal implementation of `grab::score_elements`. No public API changes.

- Absolute URL transformation is now performed internally by `dom_smoothie`.

- The `url` dependency has been removed from `dom_smoothie` for the following reasons:
  - Although an excellent crate, its features are excessive for `dom_smoothie`. It requires only `is_absolute_url` and `to_absolute_url` functionality.
  - MSRV issues: `url` requires Rust 1.63, but its `idna` dependencies require 1.82. This would prevent `dom_smoothie` 
  from building on older Rust versions, and disabling these dependencies is cumbersome.
- **Breaking**: `ReadabilityError::BadDocumentURL` is now a unit variant (`BadDocumentURL`) instead of a tuple variant. Update downstream pattern matches accordingly.
- **Breaking**: `Readability::doc_url` type changed from `Option<url::Url>` to `Option<String>`. Update code accessing this public field.
- Set MSRV to 1.75.
- Downgraded `phf` to `0.11.3` to prevent duplicate dependencies (`cssparser`, `selectors`, `web_atoms`).
- Updated `dom_query` version from `0.21.0` to `0.22.0`.

### Fixed
- Fixed `Readability::fix_relative_uris` behavior when handling srcset\'s item without a condition (e.g., `image.jpg` instead of `image.jpg 2x`).
- Fixed `MATCHER_SOURCES`, which previously contained a typo (`sources` instead of `source`).
- Fixed metadata key typos:
  - `META_MOD_TIME_KEYS`: "dcterms.modifie" -> "dcterms.modified"
  - `META_TITLE_KEYS`: "dcterm:title" -> "dcterms:title"
  - `META_EXCERPT_KEYS`: "dcterm:description" -> "dcterms:description"
  - `META_PROPERTY_PREFIXES`: "dcterm" -> "dcterms"
  - `META_NAME_PREFIXES`: "dcterm" -> "dcterms"

## [0.11.2] - 2025-08-09

### Changed
- Minor internal code changes.
- Updated `dom_query` version from `0.19.2` to `0.20.1`.

## [0.11.1] - 2025-07-08

### Changed
- Updated `dom_query` version from `0.18.0` to `0.19.2`.


## [0.11.0] - 2025-04-30

### Changed
- Updated `dom_query` version to `0.18.0`.
- Updated codebase to match latest changes (a07e62c) in [mozilla/readability](https://github.com/mozilla/readability) library.
- Minor internal code changes.


## [0.10.0] - 2025-04-01

### Added
- Added the `Config::min_score_to_adjust` option, which allows controlling the minimum score required for adjustment during the scoring process. Only nodes with a score higher than this value will be adjusted by their link density. Thus, the higher the value, the faster the scoring process.
- Implemented the `aho-corasick` feature, enabling the use of the `aho-corasick` crate for defining unlikely candidates and for the node scoring process. This can speed up the overall parsing process by 5-10% in some cases, at the cost of slightly higher memory usage and an increase in binary size.

### Changed
- Improved the internal function `fix_lazy_images` to better detect occurrences of `lazy` as a substring within an element's `class` attribute.
- Optimized the internal function `should_clean_conditionally` to improve performance.
- Minor internal code changes.
- Changed the default allocator for `dom-smoothie-js` from `alloc_cat` to `lol_alloc` because `lol_alloc` is licensed under **MIT**, whereas `alloc_cat` is not.

## [0.9.0] - 2025-03-17

### Added
- Added `Readability::parse_with_policy` method, which performs one attempt to extract relevant content from an HTML document with `ParsePolicy`. This method consumes **significantly** less memory than `Readability::parse` but it is also less precise, as `Readability::parse` is able to perform more than one attempt.
- Added the `dom_smoothie_js::Readability::parse_with_policy` method, a wrapper around `dom_smoothie::Readability::parse_with_policy`.

### Changed
- Ignoring `svg` elements during pre-filtering and element collection for scoring, improving performance for documents with many `svg` elements.

### Fixed
- Fixed the `get_row_and_col_count` function, which determines the number of rows and columns. Skipped counting `rowspan` since it is meaningless.

## [0.8.0] - 2025-03-10

### Changed
- Link elements (`<a>`) without an `href` attribute and without child nodes are now removed from the article content during post-processing.
- Changed how phrasing content determines wrapping some `<div>` element children with a `<p>` element. Now the element must contain some nodes to be wrapped.
- Updated `dom_query`'s version to `0.16.0`.

## [0.7.0] - 2025-03-03

### Added
- `Readability::parse` can now output text as `Markdown` in `Article::text_content` when `Config::text_mode` is set to `TextMode::Markdown`.

### Changed
- Update `dom_query`'s version to `0.15.1`.
- Minor code changes.

## [0.6.1] - 2025-02-16

### Changed
- Update `dom_query`'s version to `0.14.0` which brings performance improvements and improves the accuracy of the `NodeRef::formatted_text` method.
- Code optimizations, which improve the performance of the `Readability::parse` method.


## [0.6.0] - 2025-02-13

### Fixed
- Avoid a potential underflow of `orig_wc - 1` in `Readability::get_article_title`, which causes a panic when the `<title>` element contains only the `/` character. (Fix by @rMazeiks).

### Added
- Introduced `dom-smoothie-js` a sub-crate that wraps the `dom_smoothie` for use in a JS environment.

### Changed
- Switch from using regular expressions to equivalent matching functions.

## [0.5.1] - 2025-02-08

### Changed
- Updated `dom_query` version to `0.13.3`, which improves the accuracy of the `NodeRef::formatted_text` method.

### Fixed
- `Config` now implements `#[serde(default)]`. This change makes it more convenient to work with serde by removing the need to explicitly set every value in `Config`.

## [0.5.0] - 2025-02-06

### Added
- Introducing the `Config::candidate_select_mode`: this mode determines whether the top candidate is adjusted in the [Readability.js](https://github.com/mozilla/readability)  order or using the crate's exclusive implementation.
- Introducing the `Config::text_mode`: this mode determines whether the text is formatted or not. The default is `TextMode::Raw`, which is completely compatible with previous versions of this crate.

### Changed
- Changed the `Readability::grab_article` method implementation to retain only the best attempt among failed attempts, instead of keeping all of them until the exit.
- Internal code optimizations aimed to reduce execution time.
- **Breaking** Revised document filtering. Since most of the filtering is now separated from extracting elements for scoring. This applies to `Metadata.byline`, which previously could incorrectly assign a commentator as the article's author or leave it missing altogether. In the `mozilla/readability` test pages, I've encountered cases where this happened because `Readability` failed to extract readable content on the first iteration. The removal of duplicate `Metadata.title` elements is handled more accurately, reducing redundancy and improving document clarity.
- If Metadata.byline was assigned while grabbing the article, it will be normalized (no new lines or trailing spaces).

### Fixed
- Corrected handling of manually created `p` elements for scores. Previously, these elements were sometimes omitted.
- Skipped ancestor assignment for elements beyond the `body` element. Previously, these elements may have been incorrectly assigned to the root element, which has no parent, causing a runtime panic.


## [0.4.0] - 2025-01-21

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
