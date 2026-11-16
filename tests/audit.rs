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

#[test]
fn test_colspan_overflow_does_not_panic() {
    let max = usize::MAX.to_string();
    let cases = ["1", &max, "99999999999999999999999"];
    for case in cases {
        let html = format!(
            r#"<html><body><article>
            <p>First paragraph with enough text to be considered content by the extractor. It keeps going for a while.</p>
            <table><tr><td colspan="{case}">a</td><td colspan="{case}">b</td></tr></table>
            <p>Second paragraph with enough text to be considered content by the extractor. It also keeps going for a while.</p>
        </article></body></html>"#
        );
        let mut ra = Readability::new(html.as_str(), None, Some(Config::default())).unwrap();
        assert!(ra.parse().is_ok());
    }
}

/// A wrapper's class must not decide the fate of the `<pre>` it holds (issue #226).
#[test]
fn test_pre_in_utility_class_wrapper_survives_cleaning() {
    let p = "<p>This paragraph is long enough to count as article text, with several commas, \
             clauses, and details, so that the scoring treats the container as the main content \
             of the page rather than as navigation.</p>";
    for class in ["", "group", "scroll-mt-2", "md:hidden", "overflow-hidden"] {
        let attr = if class.is_empty() {
            String::new()
        } else {
            format!(r#"class="{class}""#)
        };
        let html = format!(
            "<html><body><article><h1>Title</h1>{p}{p}\
             <div {attr}><pre><code>sudo apt install x</code></pre></div>\
             {p}{p}</article></body></html>"
        );
        let mut ra = Readability::new(html.as_str(), None, Some(Config::default())).unwrap();
        let article = ra.parse().unwrap();
        assert!(
            article.content.contains("sudo apt install x"),
            "<pre> dropped for {attr}"
        );
    }
}

/// Keeping `<pre>` wrappers must not resurrect elements hidden by an attribute or an inline style:
/// those are stripped before scoring.
#[test]
fn test_hidden_pre_wrapper_is_dropped() {
    let p = "<p>This paragraph is long enough to count as article text, with several commas, \
             clauses, and details, so that the scoring treats the container as the main content \
             of the page rather than as navigation.</p>";
    let wrappers = [
        "<div hidden><pre><code>sudo apt install x</code></pre></div>",
        r#"<div style="display:none"><pre><code>sudo apt install x</code></pre></div>"#,
    ];
    for wrapper in wrappers {
        let html = format!(
            "<html><body><article><h1>Title</h1>{p}{p}{wrapper}{p}{p}</article></body></html>"
        );
        let mut ra = Readability::new(html.as_str(), None, Some(Config::default())).unwrap();
        let article = ra.parse().unwrap();
        assert!(
            !article.content.contains("sudo apt install x"),
            "hidden <pre> kept for {wrapper}"
        );
    }
}

/// Commas in `<pre>` are code syntax, not prose clauses: a comma-heavy code sample must not
/// outscore the article and pull its container into the extracted content.
#[test]
fn test_skip_pre_comma_scoring() {
    let p = "<p>The committee reviewed the proposal over three sessions and agreed to fund the new \
             library wing. The vote passed with a clear majority, and work will start next spring.</p>";
    let items: Vec<String> = (0..120).map(|i| format!("item{i}")).collect();
    let code = format!("const sample = [{}];", items.join(", "));
    let html = format!(
        "<html><body><div id=\"page\"><div id=\"story\"><h1>Title</h1>{p}{p}{p}</div>\
         <div class=\"demo\"><div><pre>{code}</pre></div></div></div></body></html>"
    );
    let mut ra = Readability::new(html.as_str(), None, Some(Config::default())).unwrap();
    let article = ra.parse().unwrap();
    assert!(article.content.contains("The vote passed with a clear majority"));
    assert!(!article.content.contains("const sample"));
}
