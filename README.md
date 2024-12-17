# DOM_SMOOTHIE

> A Rust crate for extracting relevant content from web pages.

DOM_SMOOTHIE closely follows the implementation of [readability.js](https://github.com/mozilla/readability), bringing its functionality to Rust.


## Examples

### Basic Example

```rust
use std::error::Error;

use dom_query::Document;
use dom_smoothie::Readability;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = dom_smoothie::Config {
        classes_to_preserve: vec!["caption".into()],
        ..Default::default()
    };

    let html = include_str!("../test-pages/ok/001/source.html");
    let doc_url = Some("http://fakehost/test/");

    let mut readability = Readability::new(html, doc_url, Some(cfg))?;

    println!("Title: {}", &readability.get_article_title());

    let article = readability.parse()?;

    println!("Content:\n {}", article.content);
    Ok(())
}
```


## License

Licensed under MIT ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).

## Contribution

Any contribution intentionally submitted for inclusion in this project will be licensed under the MIT license, without any additional terms or conditions.
