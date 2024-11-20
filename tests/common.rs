use std::{fs, path::Path};

use dom_query::{Document, Matcher};
use dom_smoothie::Readability;

use once_cell::sync::Lazy;
pub(crate) static R_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("#readability-page-1").unwrap());

pub fn test_readability<P>(test_path: P, host: Option<&str>)
where
    P: AsRef<Path>,
{
    let base_path = test_path.as_ref();
    let source_path = base_path.join("source.html");
    let expected_path = base_path.join("expected.html");

    let source_contents = fs::read_to_string(source_path).unwrap();
    let cfg = dom_smoothie::Config {
        classes_to_preserve: vec!["caption".into()],
        ..Default::default()
    };
    let mut r = Readability::new(source_contents, host, Some(cfg));
    let article = r.parse();

    let contents = article.content;
    let article_doc = Document::from(contents);

    let expected_contents = fs::read_to_string(expected_path).unwrap();
    let expected_doc = Document::from(expected_contents);

    let a_html = article_doc
        .select_single_matcher(&R_MATCHER)
        .html()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("");

    let e_html = expected_doc
        .select_single_matcher(&R_MATCHER)
        .html()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("");

    assert_eq!(
        a_html,
        e_html,
        "parsed contents for test {} do not match with expected content",
        test_path.as_ref().display()
    );
}
