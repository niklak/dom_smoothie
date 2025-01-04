use std::fs;

mod common;

use common::test_metadata;

#[test]
fn test_metadata_last_fail() {
    test_metadata(
        "./test-pages/readability/mozilla-1",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn table_test_metadata() {
    let paths = fs::read_dir("./test-pages/ok").unwrap();
    for p in paths {
        let pp = p.unwrap().path();
        test_metadata(pp, Some("http://fakehost/test/"));
    }
}
