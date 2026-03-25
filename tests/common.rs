#![allow(dead_code)]

use std::{
    fs, io,
    path::{Path, PathBuf},
};

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

#[allow(unused_macros)]
macro_rules! test_data {
    ($test_path:expr, $expected_file:expr) => {
        TestData::from_path($test_path, None, $expected_file).unwrap()
    };
}

#[allow(unused_macros)]
macro_rules! include_test_data {
    ($test_path:expr, $source_file:expr, $expected_file:expr) => {
        TestData::new(
            $test_path,
            include_str!($source_file).to_string(),
            include_str!($expected_file).to_string(),
        )
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

pub struct TestData {
    path: PathBuf,
    source_contents: String,
    expected_contents: String,
}

impl TestData {
    /// new creates a new instance of `TestData`.
    pub fn new<P>(test_path: P, source_contents: String, expected_contents: String) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            path: test_path.as_ref().to_path_buf(),
            source_contents,
            expected_contents,
        }
    }

    pub fn from_path<P>(
        test_path: P,
        source_file: Option<&str>,
        expected_file: &str,
    ) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let base_path = test_path.as_ref();
        let source_file = source_file.unwrap_or("source.html");
        let source_path = base_path.join(source_file);
        let source_contents = fs::read_to_string(source_path)?;
        let expected_path = base_path.join(expected_file);
        let expected_contents = fs::read_to_string(expected_path)?;
        Ok(Self::new(test_path, source_contents, expected_contents))
    }
}

pub(crate) fn test_alt_text(data: TestData, text_mode: TextMode) {
    // for more options check the documentation
    let cfg = Config {
        candidate_select_mode: CandidateSelectMode::DomSmoothie,
        text_mode,
        ..Default::default()
    };

    let mut readability = Readability::new(data.source_contents, None, Some(cfg)).unwrap();

    let article = readability.parse().unwrap();
    let article_text = article.text_content.as_ref();
    let expected = data.expected_contents.trim();
    check!("text_content", article_text, expected, &data.path);
}

pub(crate) fn test_readability(data: TestData) {
    let doc_url = Some("http://fakehost/test/");
    let cfg = dom_smoothie::Config {
        classes_to_preserve: vec!["caption".into()],
        ..Default::default()
    };
    let mut r = Readability::new(data.source_contents, doc_url, Some(cfg)).unwrap();
    let article = r.parse().unwrap();

    let contents = article.content;
    let article_doc = Document::from(contents);

    let expected_doc = Document::from(data.expected_contents);

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

    check!("content", a_html, e_html, data.path);
}

pub fn test_metadata(data: TestData, host: Option<&str>) {
    let cfg = dom_smoothie::Config {
        classes_to_preserve: vec!["caption".into()],
        ..Default::default()
    };
    let mut r = Readability::new(data.source_contents, host, Some(cfg)).unwrap();

    let readable = r.is_probably_readable();
    let article = r.parse().unwrap();

    let exp: ExpectedMetadata = serde_json::from_str(&data.expected_contents).unwrap();

    check!("readable", &readable, &exp.readerable, data.path);
    check!("title", &article.title, &exp.title, data.path);
    check!("byline", &article.byline, &exp.byline, data.path);
    check!("excerpt", &article.excerpt, &exp.excerpt, data.path);
    check!("site_name", &article.site_name, &exp.site_name, data.path);
    check!(
        "published_time",
        &article.published_time,
        &exp.published_time,
        data.path
    );
    check!("lang", &article.lang, &exp.lang, data.path);
    check!("dir", &article.dir, &exp.dir, data.path);
    check!("image", &article.image, &exp.image, data.path);
}

pub fn test_favicon<P>(test_path: P, host: Option<&str>, expected: Option<&str>)
where
    P: AsRef<Path>,
{
    let source_path = test_path.as_ref().join("source.html");
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
        &test_path.as_ref()
    );
}
