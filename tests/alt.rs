mod common;

use std::fs;

use common::{test_alt_text, TestData};

#[test]
fn test_alt_formatted_last_fail() {
    let data = TestData::from_path("./test-pages/alt/arxiv", None, "expected_alt.txt").unwrap();

    test_alt_text(data, dom_smoothie::TextMode::Formatted);
}

#[test]
fn table_test_alt_formatted_text() {
    let paths = fs::read_dir("./test-pages/alt").unwrap();
    for p in paths {
        let pp = p.unwrap().path();
        let data = TestData::from_path(pp, None, "expected_alt.txt").unwrap();
        test_alt_text(data, dom_smoothie::TextMode::Formatted);
    }
}

#[test]
fn table_test_alt_markdown() {
    let paths = fs::read_dir("./test-pages/alt").unwrap();
    for p in paths {
        let pp = p.unwrap().path();
        let data = TestData::from_path(pp, None, "expected.md").unwrap();
        test_alt_text(data, dom_smoothie::TextMode::Markdown);
    }
}
