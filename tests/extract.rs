use dom_smoothie::{
    Config, Metadata, Readability, ReadabilityError, TextMode, dom_query::Document,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_extract_matches_parse() {
    for html in [
        include_str!("../test-pages/ok/wikipedia-2/source.html"),
        include_str!("../test-pages/ok/base-url-base-element-relative/source.html"),
        include_str!("../test-pages/readability/lazy-image-3/source.html"),
    ] {
        for text_mode in [TextMode::Raw, TextMode::Formatted, TextMode::Markdown] {
            let config = Config {
                text_mode,
                ..Default::default()
            };
            let url = Some("https://example.org/blog/post");
            let article = Readability::new(html, url, Some(config.clone()))
                .unwrap()
                .parse()
                .unwrap();
            let extracted = Readability::new(html, url, Some(config))
                .unwrap()
                .extract()
                .unwrap();
            assert_eq!(article.content, extracted.selection().html());
            let text = match text_mode {
                TextMode::Raw => extracted.root().text(),
                TextMode::Formatted => extracted.root().formatted_text(),
                TextMode::Markdown => extracted.root().md(None),
            };
            assert_eq!(article.text_content, text);
            assert_eq!(article.length, text.chars().count());
            assert_eq!(
                extracted.metadata,
                Metadata {
                    title: article.title,
                    byline: article.byline,
                    excerpt: article.excerpt,
                    site_name: article.site_name,
                    published_time: article.published_time,
                    modified_time: article.modified_time,
                    image: article.image,
                    favicon: article.favicon,
                    lang: article.lang,
                    url: article.url,
                    dir: article.dir,
                }
            );
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_extract_dom_access() {
    let html = r#"<!doctype html><html lang="en"><head><title>Example article</title>
        <meta name="author" content="Ada"></head><body><article dir="rtl">
        <h1>Example article</h1>
        <p>Enough readable words to keep this paragraph in the extracted article body.
        See <a href="docs">the docs</a> and <img src="plot.png" alt="Plot">.</p>
        <dl><dt>Term</dt><dd>Definition</dd></dl>
        <details open><summary>More</summary><p>Hidden body.</p></details>
        <table><caption>Numbers</caption><tr><th>A</th><th>B</th></tr>
        <tr><td>1</td><td>2</td></tr></table>
        </article></body></html>"#;
    let extracted = Readability::with_document(
        Document::from(html),
        Some("https://example.org/blog/post"),
        None,
    )
    .unwrap()
    .extract()
    .unwrap();
    assert_eq!(extracted.root().id, extracted.root);
    assert_eq!(extracted.metadata.title, "Example article");
    assert_eq!(extracted.metadata.byline.as_deref(), Some("Ada"));
    let body = extracted.selection();
    for (selector, expected) in [
        ("dl dt", "Term"),
        ("details summary", "More"),
        ("table caption", "Numbers"),
    ] {
        assert_eq!(body.select(selector).text().as_ref(), expected);
    }
    assert_eq!(
        body.select("a").attr("href").as_deref(),
        Some("https://example.org/blog/docs")
    );
    assert_eq!(
        body.select("img").attr("src").as_deref(),
        Some("https://example.org/blog/plot.png")
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_extract_readme_example() {
    let document = Document::from(
        "<!DOCTYPE html>\
         <html>\
             <head><title>Example</title></head>\
             <body>\
                 <article>\
                     <p>Readable content with <a href='/docs'>a link</a>.</p>\
                 </article>\
             </body>\
         </html>",
    );
    let extracted = Readability::with_document(document, Some("https://example.com/article"), None)
        .unwrap()
        .extract()
        .unwrap();

    assert_eq!(extracted.metadata.title, "Example");
    assert_eq!(
        extracted.selection().select("a").attr("href").as_deref(),
        Some("https://example.com/docs")
    );
}

#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
fn test_extract_errors() {
    for html in ["<html><body></body></html>", "<frameset></frameset>"] {
        assert!(matches!(
            Readability::new(html, None, None).unwrap().extract(),
            Err(ReadabilityError::GrabFailed)
        ));
    }
    let document = Document::fragment("<p>Content without a body.</p>");
    assert!(matches!(
        Readability::with_document(document, None, None)
            .unwrap()
            .extract(),
        Err(ReadabilityError::GrabFailed)
    ));
    let config = Config {
        max_elements_to_parse: 1,
        ..Default::default()
    };
    assert!(matches!(
        Readability::new(
            "<p>Content above the element limit.</p>",
            None,
            Some(config)
        )
        .unwrap()
        .extract(),
        Err(ReadabilityError::TooManyElements(_, 1))
    ));
}
