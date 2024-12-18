# DOM_SMOOTHIE

[![Crates.io version](https://img.shields.io/crates/v/dom_smoothie.svg?style=flat)](https://crates.io/crates/dom_smoothie)
[![Download](https://img.shields.io/crates/d/dom_smoothie.svg?style=flat)](https://crates.io/crates/dom_smoothie)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat)](https://docs.rs/dom_smoothie)
[![codecov](https://codecov.io/gh/niklak/dom_smoothie/graph/badge.svg?token=X0LB1HB90L)](https://codecov.io/gh/niklak/dom_smoothie)

[![Rust CI](https://github.com/niklak/dom_smoothie/actions/workflows/rust.yml/badge.svg)](https://github.com/niklak/dom_smoothie/actions/workflows/rust.yml)

> A Rust crate for extracting relevant content from web pages.

DOM_SMOOTHIE closely follows the implementation of [readability.js](https://github.com/mozilla/readability), bringing its functionality to Rust.


## Examples

### Basic Example

```rust
use std::error::Error;

use dom_smoothie::Readability;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = dom_smoothie::Config {
        classes_to_preserve: vec!["caption".into()],
        ..Default::default()
    };

    let html = include_str!("../test-pages/ok/001/source.html");

    let mut readability = Readability::new(html, Some("http://fakehost/test/"), Some(cfg))?;
    let article = readability.parse()?;

    println!("Title: {}", &article.title);


    println!("Content:\n {}", &article.content);
    Ok(())
}
```


## License

Licensed under MIT ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).

## Contribution

Any contribution intentionally submitted for inclusion in this project will be licensed under the MIT license, without any additional terms or conditions.
