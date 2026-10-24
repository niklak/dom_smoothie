use dom_query::Document;
use dom_smoothie::{Config, Readability};

#[test]
fn test_preserve_classes_with_css_special_chars() {
    let html = r#"<html><body><article>
        <p class="md:flex keep w-1/2">First paragraph with enough text to be considered content by the extractor. It keeps going for a while.</p>
        <p class="drop">Second paragraph with enough text to be considered content by the extractor. It also keeps going for a while.</p>
        <p class="a b a,b">Third paragraph with enough text to be considered content by the extractor. It also keeps going for a while.</p>
    </article></body></html>"#;
    let cfg = Config {
        classes_to_preserve: vec!["md:flex".into(), "w-1/2".into(), "a,b".into()],
        ..Default::default()
    };
    let mut ra = Readability::new(html, None, Some(cfg)).unwrap();
    let article = ra.parse().unwrap();
    let doc = Document::from(article.content);

    let classes: Vec<_> = doc
        .select("p")
        .nodes()
        .iter()
        .map(|p| p.attr("class"))
        .collect();
    assert_eq!(classes[0].as_deref(), Some("md:flex w-1/2"));
    assert_eq!(classes[1], None);
    assert_eq!(classes[2].as_deref(), Some("a,b"));
    assert_eq!(
        doc.select("#readability-page-1").attr("class").as_deref(),
        Some("page")
    );
}
