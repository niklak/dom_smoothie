mod common;

use std::fs;

use common::test_alt_text;

#[test]
fn test_alt_formatted_last_fail() {
    test_alt_text(
        "./test-pages/alt/arxiv",
        dom_smoothie::TextMode::Formatted,
        "expected_alt.txt",
    );
}

#[test]
fn table_test_alt_formatted_text() {
    let paths = fs::read_dir("./test-pages/alt").unwrap();
    for p in paths {
        let pp = p.unwrap().path();
        test_alt_text(pp, dom_smoothie::TextMode::Formatted, "expected_alt.txt");
    }
}

#[test]
fn table_test_alt_markdown() {
    let paths = fs::read_dir("./test-pages/alt").unwrap();
    for p in paths {
        let pp = p.unwrap().path();
        test_alt_text(pp, dom_smoothie::TextMode::Markdown, "expected.md");
    }
}
