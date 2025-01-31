mod common;

use std::fs;

use common::test_alt_formatted_text;

#[test]
fn test_alt_formattedd_last_fail() {
    test_alt_formatted_text("./test-pages/alt/arxiv");
}

#[test]
fn table_test_alt_formatted_text() {
    let paths = fs::read_dir("./test-pages/alt").unwrap();
    for p in paths {
        let pp = p.unwrap().path();
        test_alt_formatted_text(pp);
    }
}
