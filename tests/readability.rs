use std::fs;

mod common;

use common::test_readability;

#[test]
fn table_test_readability() {
    let paths = fs::read_dir("./test-pages/readability").unwrap();

    for p in paths {
        test_readability(p.unwrap().path(), Some("http://fakehost/test/"));
    }
}

#[test]
#[cfg(feature = "serde")]
fn test_serde() {
    let contents = include_str!("../test-pages/ok/base-url-base-element-relative/source.html");
    let document_url = Some("http://fakehost/test/");
    let mut ra = dom_smoothie::Readability::new(contents, document_url, None).unwrap();
    let article = ra.parse().unwrap();
    let article_json = serde_json::to_string(&article);
    assert!(article_json.is_ok());

    let article_json = article_json.unwrap();
    let article_copy: dom_smoothie::Article = serde_json::from_str(&article_json).unwrap();
    assert_eq!(article.content, article_copy.content);
}