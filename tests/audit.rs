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

/// `is_unlikely_candidate` exempts `<code>` elements and `<table>` elements whose only unlikely
/// match is `"header"` (such as `ltx_guessed_headers`), while still stripping sidebar/navigation
/// tables such as `table.sidebar`.
#[test]
fn test_header_class_table_is_kept() {
    let p = "<p>The committee reviewed the proposal over three sessions, weighing the costs against \
             the expected benefits for local residents, and finally agreed to fund the new library \
             wing, the reading garden, and the evening programs for children.</p>";
    let html = format!(
        r#"<html><body><article><h1>Title</h1>{p}{p}
        <table class="sidebar nomobile">
          <tbody><tr><th><a href="/series">Sidebar Series</a></th></tr></tbody>
        </table>
        <figure class="ltx_table">
          <table class="ltx_tabular ltx_guessed_headers">
            <thead><tr><th>Model</th><th>BLEU EN-DE</th><th>BLEU EN-FR</th></tr></thead>
            <tbody>
              <tr><td>ByteNet</td><td>23.75</td><td>39.2</td></tr>
              <tr><td>Transformer</td><td>28.40</td><td>41.8</td></tr>
            </tbody>
          </table>
        </figure>
        <p>Run <code class="menu-header-cmd">cargo test --all</code> to verify.</p>
        {p}</article></body></html>"#
    );
    let mut ra = Readability::new(html.as_str(), None, Some(Config::default())).unwrap();
    let article = ra.parse().unwrap();
    assert!(article.content.contains("ByteNet"));
    assert!(article.content.contains("Transformer"));
    assert!(article.content.contains("cargo test --all"));
    assert!(!article.content.contains("Sidebar Series"));
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

/// Class names that merely contain `scroll` (`scroll_photo_view`, `overflow-x-scroll`, `scroll-mt-2`)
/// are layout utilities, not boilerplate: a container with such a class keeps its article text.
#[test]
fn test_scroll_container_is_kept() {
    let p = "<p>The committee reviewed the proposal over three sessions, weighing the costs against \
             the expected benefits for local residents, and finally agreed to fund the new library \
             wing, the reading garden, and the evening programs for children.</p>";
    let quote = "The final report was published inside the scroll container.";
    for class in ["scroll_photo_view", "overflow-x-scroll", "scroll-mt-2"] {
        let html = format!(
            "<html><body><div id=\"content\"><h1>Title</h1>\
             <div class=\"{class}\"><p>{quote}</p>{p}</div>{p}{p}{p}</div></body></html>"
        );
        let mut ra = Readability::new(html.as_str(), None, Some(Config::default())).unwrap();
        let article = ra.parse().unwrap();
        assert!(
            article.content.contains(quote),
            "text inside class {class:?} was dropped"
        );
    }
}
