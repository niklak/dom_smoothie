use std::fs;

mod common;

use common::test_metadata;

#[test]
fn test_metadata_last_fail() {
    test_metadata(
        "./test-pages/readability/bbc-1",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn table_test_metadata() {
    let source_dirs = ["./test-pages/readability", "./test-pages/ok"];
    for d in source_dirs {
        let paths = fs::read_dir(d).unwrap();
        for p in paths {
            let pp = p.unwrap().path();
            test_metadata(pp, Some("http://fakehost/test/"));
        }
    }
}
