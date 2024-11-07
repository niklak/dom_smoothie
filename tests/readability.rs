use std::{fs, path::Path};

use dom_query::{Document, Matcher};
use dom_smoothie::Readability;

use once_cell::sync::Lazy;
pub(crate) static R_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("#readability-page-1").unwrap());

fn test_readability<P>(test_path: P, host: Option<&str>)
where
    P: AsRef<Path>,
{
    let base_path = test_path.as_ref();
    let source_path = base_path.join("source.html");
    let expected_path = base_path.join("expected.html");

    let source_contents = fs::read_to_string(source_path).unwrap();
    let mut r = Readability::new(source_contents, host);
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

#[test]
fn test_001() {
    test_readability("test-pages/readability/001/", Some("http://fakehost"));
}

#[test]
fn test_002() {
    test_readability("test-pages/readability/002/", Some("http://fakehost"));
}

#[test]
fn test_003() {
    test_readability(
        "test-pages/readability/003-metadata-preferred/",
        Some("http://fakehost"),
    );
}

#[test]
fn test_004() {
    test_readability(
        "test-pages/readability/004-metadata-space-separated-properties/",
        Some("http://fakehost"),
    );
}

#[test]
fn test_005() {
    //TODO: important can't pass
    test_readability(
        "test-pages/readability/005-unescape-html-entities/",
        Some("http://fakehost"),
    );
}

#[test]
fn test_aclu() {
    test_readability("test-pages/readability/aclu/", Some("http://fakehost"));
}

#[test]
fn test_aktualne() {
    //TODO: important can't pass
    test_readability("test-pages/readability/aktualne/", Some("http://fakehost"));
}

#[test]
fn test_replace_brs() {
    test_readability(
        "test-pages/readability/replace-brs/",
        Some("http://fakehost"),
    );
}

/*#[test]
fn table_test_readability() {

    let paths = fs::read_dir("./test-pages/readability").unwrap();

    for p in paths {
        test_readability(p.unwrap().path(), Some("http://fakehost"));
    }
}*/
