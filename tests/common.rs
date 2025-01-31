#![allow(dead_code)]
use std::{fs, path::Path};

use dom_query::{Document, Matcher};
use dom_smoothie::{CandidateSelectMode, Config, Readability, TextMode};

use once_cell::sync::Lazy;
pub(crate) static R_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("#readability-page-1").unwrap());

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExpectedMetadata {
    title: String,
    byline: Option<String>,
    excerpt: Option<String>,
    site_name: Option<String>,
    published_time: Option<String>,
    lang: Option<String>,
    dir: Option<String>,
    readerable: bool,
}

pub(crate) fn test_alt_formatted_text<P>(test_path: P)
where
    P: AsRef<Path>,
{
    let base_path = test_path.as_ref();
    let source_path = base_path.join("source.html");
    let expected_path = base_path.join("expected_alt.txt");
    // for more options check the documentation
    let cfg = Config {
        candidate_select_mode: CandidateSelectMode::DomSmoothie,
        text_mode: TextMode::Formatted,
        ..Default::default()
    };

    let source_contents = fs::read_to_string(source_path).unwrap();
    let mut readability = Readability::new(source_contents, None, Some(cfg)).unwrap();

    let article = readability.parse().unwrap();
    let expected_contents = fs::read_to_string(expected_path).unwrap();
    let article_text = article.text_content.as_ref();
    assert_eq!(article_text, expected_contents.trim())
}

pub(crate) fn test_readability<P>(test_path: P)
where
    P: AsRef<Path>,
{
    let doc_url = Some("http://fakehost/test/");
    let base_path = test_path.as_ref();
    let source_path = base_path.join("source.html");
    let expected_path = base_path.join("expected.html");

    let source_contents = fs::read_to_string(source_path).unwrap();
    let cfg = dom_smoothie::Config {
        classes_to_preserve: vec!["caption".into()],
        ..Default::default()
    };
    let mut r = Readability::new(source_contents, doc_url, Some(cfg)).unwrap();
    let article = r.parse().unwrap();

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

pub fn test_metadata<P>(test_path: P, host: Option<&str>)
where
    P: AsRef<Path>,
{
    let base_path = test_path.as_ref();
    let source_path = base_path.join("source.html");

    let source_contents = fs::read_to_string(source_path).unwrap();
    let cfg = dom_smoothie::Config {
        classes_to_preserve: vec!["caption".into()],
        ..Default::default()
    };
    let mut r = Readability::new(source_contents, host, Some(cfg)).unwrap();

    let readable = r.is_probably_readable();

    let article = r.parse().unwrap();

    let expected_metadata_path = base_path.join("expected-metadata.json");
    let meta_contents = fs::read_to_string(expected_metadata_path).unwrap();
    let expected: ExpectedMetadata = serde_json::from_str(&meta_contents).unwrap();

    assert_eq!(
        readable, expected.readerable,
        "readerable does not match expected"
    );

    assert_eq!(
        article.title, expected.title,
        "title does not match expected"
    );
    assert_eq!(
        article.byline, expected.byline,
        "byline does not match expected"
    );
    assert_eq!(
        article.excerpt, expected.excerpt,
        "excerpt does not match expected"
    );
    assert_eq!(
        article.site_name, expected.site_name,
        "site_name does not match expected"
    );
    assert_eq!(
        article.published_time, expected.published_time,
        "published_time does not match expected"
    );
    assert_eq!(article.lang, expected.lang, "lang does not match expected");
    assert_eq!(article.dir, expected.dir, "dirs does not match expected");
}
