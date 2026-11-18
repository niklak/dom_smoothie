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

/// `<math>` elements with a non-empty `<annotation encoding="application/x-tex">` replace their
/// inner presentation children with the `$...$` (inline) or `$$...$$` (`display="block"`) LaTeX
/// source instead of concatenating the presentation tokens and annotation text together.
#[test]
fn test_math_tex_annotation() {
    let p = "<p>The committee reviewed the proposal over three sessions, weighing the costs against \
             the expected benefits for local residents, and finally agreed to fund the new library \
             wing, the reading garden, and the evening programs for children.</p>";
    let html = format!(
        r#"<html><body><article><h1>Title</h1>{p}{p}
        <p>The hidden state <math display="inline"><semantics><msub><mi>h</mi><mi>t</mi></msub><annotation encoding="application/x-tex">h_{{t}}</annotation></semantics></math>
        depends on <math><mi>x</mi><annotation encoding="application/x-tex">   </annotation></math>.</p>
        <p><math display="block"><semantics><mrow><mi>E</mi><mo>=</mo><mi>m</mi><msup><mi>c</mi><mn>2</mn></msup></mrow><annotation encoding="application/x-tex">E = mc^2</annotation></semantics></math></p>
        {p}</article></body></html>"#
    );
    let mut ra = Readability::new(html.as_str(), None, Some(Config::default())).unwrap();
    let article = ra.parse().unwrap();
    assert!(article.text_content.contains("hidden state $h_{t}$"));
    assert!(!article.text_content.contains("hth_{t}"));
    assert!(article.text_content.contains("$$E = mc^2$$"));
    assert!(article.text_content.contains("depends on x."));
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
