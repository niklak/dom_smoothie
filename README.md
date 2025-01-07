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
    // Readability supplies an optional `Config`. If `cfg` is omitted, then a default `Config` instance will be used.
    // Readability also supplies an optional `document_url` parameter, which may be used to transform relative URLs into absolute URLs.
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
    // To check out the html content of the article please have a look at `./test-pages/rustwiki_2024_result.html`
    // println!("HTML Content: {}", article.content);

    // Skipping article.text_content since it is too large.
    // To check out the html content of the article please have a look at `./test-pages/rustwiki_2024_result.txt`
    //println!("Text Content: {}", article.text_content);

    Ok(())
}
```
</details>


## License

Licensed under MIT ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).

## Contribution

Any contribution intentionally submitted for inclusion in this project will be licensed under the MIT license, without any additional terms or conditions.
