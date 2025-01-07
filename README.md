# DOM_SMOOTHIE

[![Crates.io version](https://img.shields.io/crates/v/dom_smoothie.svg?style=flat)](https://crates.io/crates/dom_smoothie)
[![Download](https://img.shields.io/crates/d/dom_smoothie.svg?style=flat)](https://crates.io/crates/dom_smoothie)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat)](https://docs.rs/dom_smoothie)
[![codecov](https://codecov.io/gh/niklak/dom_smoothie/graph/badge.svg?token=X0LB1HB90L)](https://codecov.io/gh/niklak/dom_smoothie)

[![Rust CI](https://github.com/niklak/dom_smoothie/actions/workflows/rust.yml/badge.svg)](https://github.com/niklak/dom_smoothie/actions/workflows/rust.yml)

> A Rust crate for extracting readable content from web pages.

**dom_smoothie** closely follows the implementation of [readability.js](https://github.com/mozilla/readability), bringing its functionality to Rust.


## Examples


<details>
    <summary><b>Readability::parse — a basic example</b></summary>


```rust
use std::error::Error;

use dom_smoothie::{Article, Config, Readability};

fn main() -> Result<(), Box<dyn Error>> {
    let html = include_str!("../test-pages/rustwiki_2024.html");
    let document_url = "https://en.wikipedia.org/wiki/Rust_(programming_language)";

    // for more options check the documentation
    let cfg = Config {
        max_elements_to_parse: 9000,
        ..Default::default()
    };
    // Readability supplies an optional `Config`. If `cfg` is omitted, 
    // then a default `Config` instance will be used.
    // Readability also supplies an optional `document_url` parameter, 
    // which may be used to transform relative URLs into absolute URLs.
    let mut readability = Readability::new(html, Some(document_url), Some(cfg))?;

    let article: Article = readability.parse()?;

    println!("{:<15} {}","Title:", article.title);
    println!("{:<15} {:?}","Byline:", article.byline);
    println!("{:<15} {}","Length:", article.length);
    println!("{:<15} {:?}","Excerpt:", article.excerpt);
    println!("{:<15} {:?}","Site Name:", article.site_name);
    println!("{:<15} {:?}", "Dir:", article.dir);
    println!("{:<15} {:?}","Published Time:", article.published_time);
    println!("{:<15} {:?}","Modified Time:", article.modified_time);
    println!("{:<15} {:?}","Image:", article.image);
    // This uri can be taken only from ld+json
    println!("{:<15} {:?}","URL", article.url);

    // Skipping article.content since it is too large.
    // To check out the html content of the article please have a look at
    // `./test-pages/rustwiki_2024_result.html`
    // println!("HTML Content: {}", article.content);

    // Skipping article.text_content since it is too large.
    // To check out the html content of the article please have a look at 
    // `./test-pages/rustwiki_2024_result.txt`
    //println!("Text Content: {}", article.text_content);

    // Right now, `text_content` provides almost the same result 
    // as readability.js, which is far from perfect. 
    // It may squash words together if element nodes don't have a whitespace before closing, 
    // and currently, I have no definitive opinion on this matter.

    Ok(())
}
```
</details>


<details>
    <summary><b>Parsing only metadata</b></summary>


```rust
use std::error::Error;

use dom_smoothie::{Metadata, Config, Readability};

fn main() -> Result<(), Box<dyn Error>> {
    let html = include_str!("../test-pages/rustwiki_2024.html");

    let cfg = Config {
        // parsing `ld+json` may be skipped
        disable_json_ld: false,
        ..Default::default()
    };

    // You can parse only metadata without parsing the article content
    let readability = Readability::new(html, None, Some(cfg))?;

    // <script type="application/ld+json"> may contain some useful information, 
    // but usually it is not enough.
    let ld_meta: Option<Metadata> = readability.parse_json_ld();

    if let Some(ref meta) = ld_meta {
        println!("LD META: {:#?}", meta);
    }

    println!("\n=============\n");
    // Under the hood, `Readability::parse` passes the metadata obtained from `Readability::parse_json_ld` 
    // as the basis to `Readability::get_article_metadata`. But this is not necessary.
    let meta = readability.get_article_metadata(ld_meta);
    println!("META: {:#?}", &meta);

    // Some fields of Metadata may be missing because they can be assigned
    // during the Readability::parse process.
    // This applies to `excerpt`, `byline`, and `dir`.
    Ok(())
}
```
</details>

<details>
    <summary><b>Parsing only article`s title</b></summary>


```rust
use std::error::Error;

use dom_query::Document;
use dom_smoothie::Readability;

fn main() -> Result<(), Box<dyn Error>> {
    let html = include_str!("../test-pages/rustwiki_2024.html");

    let doc: Document = dom_query::Document::from(html);

    // You can parse only the metadata without parsing the article content.
    let readability: Readability = Readability::with_document(doc, None, None)?;
    
    // Parse only the title without extracting the full content.
    let title: tendril::Tendril<tendril::fmt::UTF8> = readability.get_article_title();
    assert_eq!(title, " Rust (programming language) - Wikipedia".into());
    
    // However, this title may differ from `metadata.title`,
    // as `metadata.title` first attempts to extract the title from the metadata
    // and falls back to `Readability::get_article_title` if unavailable.
    println!("Title: {}", title);

    Ok(())
}
```
</details>


<details>
    <summary><b>Checking if content is readable</b></summary>


```rust
use std::error::Error;

use dom_smoothie::{Article, Readability};

fn main() -> Result<(), Box<dyn Error>> {
    let html = include_str!("../test-pages/rustwiki_2024.html");

    let mut readability = Readability::new(html, None, None)?;

    // There is a way to perform a quick check to determine 
    // if the document is readable before cleaning and parsing it.
    // After calling `Readability::parse`, it may show different results,
    // but calling it after parsing would be nonsensical.

    // You can specify content's min_score and min_content_length.
    if readability.is_probably_readable(Some(20.0), Some(140)) {
        let article: Article = readability.parse()?;
        println!("{:<15} {}", "Title:", article.title);
        println!("{:<15} {:?}", "Byline:", article.byline);
        println!("{:<15} {:?}", "Site Name:", article.site_name);
        println!("{:<15} {:?}", "URL", article.url);
    }

    // This is the same as:
    /*
    let doc = dom_query::Document::from(html);

    if is_probably_readable(&doc, Some(20.0), Some(140)) {

    }
    */

    Ok(())
}
```
</details>

## License

Licensed under MIT ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).

## Contribution

Any contribution intentionally submitted for inclusion in this project will be licensed under the MIT license, without any additional terms or conditions.
