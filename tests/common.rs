#![allow(dead_code)]
use std::{fs, io, path::Path};

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

pub struct TestData {
    source_contents: String,
    expected_contents: String,
}

impl TestData {
    pub fn new(source_contents: String, expected_contents: String) -> Self{
        Self {source_contents, expected_contents}
    }
    
    pub fn from_path<P>(test_path: P, source_file: Option<&str>, expected_file: &str)  -> io::Result<Self>  where P: AsRef<Path>{
        let source_file = source_file.unwrap_or("source.html");
        let base_path = test_path.as_ref();
        let source_path = base_path.join(source_file);
        let source_contents = fs::read_to_string(source_path)?;
        let expected_path = base_path.join(expected_file);
        let expected_contents = fs::read_to_string(expected_path)?;
        Ok(Self {source_contents, expected_contents})
    }
}

pub(crate) fn test_alt_text<P>(test_path: P, text_mode: TextMode, expected_filename: &str)
where
    P: AsRef<Path>,
{
    let data = TestData::from_path(&test_path, None, expected_filename).unwrap();

    // for more options check the documentation
    let cfg = Config {
        candidate_select_mode: CandidateSelectMode::DomSmoothie,
        text_mode,
        ..Default::default()
    };

    let mut readability = Readability::new(data.source_contents, None, Some(cfg)).unwrap();

    let article = readability.parse().unwrap();
    let article_text = article.text_content.as_ref();

    check!(
        "text_content",
        data.expected_contents.trim(),
        article_text,
        &test_path.as_ref()
    );
}

pub(crate) fn test_readability<P>(test_path: P)
where
    P: AsRef<Path>,
{
    let data = TestData::from_path(&test_path, None, "expected.html").unwrap();
    
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

    check!("content", a_html, e_html, &test_path.as_ref());
}

pub fn test_metadata<P>(test_path: P, host: Option<&str>)
where
    P: AsRef<Path>,
{
    let data = TestData::from_path(&test_path, None, "expected-metadata.json").unwrap();
    
    let cfg = dom_smoothie::Config {
        classes_to_preserve: vec!["caption".into()],
        ..Default::default()
    };
    let mut r = Readability::new(data.source_contents, host, Some(cfg)).unwrap();

    let readable = r.is_probably_readable();
    let article = r.parse().unwrap();


    let exp: ExpectedMetadata = serde_json::from_str(&data.expected_contents).unwrap();

    check!("readable", &readable, &exp.readerable, &test_path.as_ref());
    check!("title", &article.title, &exp.title, &test_path.as_ref());
    check!("byline", &article.byline, &exp.byline, &test_path.as_ref());
    check!("excerpt", &article.excerpt, &exp.excerpt, &test_path.as_ref());
    check!("site_name", &article.site_name, &exp.site_name, &test_path.as_ref());
    check!(
        "published_time",
        &article.published_time,
        &exp.published_time,
        &test_path.as_ref()
    );
    check!("lang", &article.lang, &exp.lang, &test_path.as_ref());
    check!("dir", &article.dir, &exp.dir, &test_path.as_ref());
    check!("image", &article.image, &exp.image, &test_path.as_ref());
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
        &test_path.as_ref()
    );
}
