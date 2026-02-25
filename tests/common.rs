#![allow(dead_code)]
use std::{fs, path::Path};

use dom_query::{Document, Matcher};
use dom_smoothie::{CandidateSelectMode, Config, Readability, TextMode};

use once_cell::sync::Lazy;
pub(crate) static R_MATCHER: Lazy<Matcher> =
    Lazy::new(|| Matcher::new("#readability-page-1").unwrap());

macro_rules! check {
    ($field:expr, $left:expr, $right:expr, $path:expr) => {
        assert_eq!(
            $left,
            $right,
            "Mismatch in field '{}' (test: {})",
            $field,
            $path.display()
        );
    };
}

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
    image: Option<String>,
}

pub(crate) fn test_alt_text<P>(test_path: P, text_mode: TextMode, expected_filename: &str)
where
    P: AsRef<Path>,
{
    let base_path = test_path.as_ref();
    let source_path = base_path.join("source.html");
    let expected_path = base_path.join(expected_filename);
    // for more options check the documentation
    let cfg = Config {
        candidate_select_mode: CandidateSelectMode::DomSmoothie,
        text_mode,
        ..Default::default()
    };

    let source_contents = fs::read_to_string(source_path).unwrap();
    let mut readability = Readability::new(source_contents, None, Some(cfg)).unwrap();

    let article = readability.parse().unwrap();
    let expected_contents = fs::read_to_string(expected_path).unwrap();
    let article_text = article.text_content.as_ref();

    check!(
        "text_content",
        article_text,
        expected_contents.trim(),
        base_path
    );
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

    check!("content", a_html, e_html, base_path);
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
    let exp: ExpectedMetadata = serde_json::from_str(&meta_contents).unwrap();

    check!("readable", &readable, &exp.readerable, &base_path);
    check!("title", &article.title, &exp.title, &base_path);
    check!("byline", &article.byline, &exp.byline, &base_path);
    check!("excerpt", &article.excerpt, &exp.excerpt, &base_path);
    check!("site_name", &article.site_name, &exp.site_name, &base_path);
    check!(
        "published_time",
        &article.published_time,
        &exp.published_time,
        &base_path
    );
    check!("lang", &article.lang, &exp.lang, &base_path);
    check!("dir", &article.dir, &exp.dir, &base_path);
    check!("image", &article.image, &exp.image, &base_path);
}

pub fn test_favicon<P>(test_path: P, host: Option<&str>, expected: Option<&str>)
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
    let r = Readability::new(source_contents, host, Some(cfg)).unwrap();

    let metadata = r.get_article_metadata(None);

    check!(
        "favicon",
        &metadata.favicon,
        &expected.map(|s| s.to_string()),
        &base_path
    );
}
