use std::fs;

#[macro_use]
mod common;

use common::{test_metadata, TestData};

#[test]
fn test_metadata_last_fail() {
    test_metadata(
        test_data!(
            "./test-pages/readability/title-en-dash",
            "expected-metadata.json"
        ),
        Some("http://fakehost/test/"),
    );
}

#[test]
fn table_test_metadata() {
    let source_dirs = ["./test-pages/readability", "./test-pages/ok"];
    for d in source_dirs {
        let paths = fs::read_dir(d).unwrap();
        for p in paths {
            let pb = p.unwrap().path();
            let data = test_data!(pb, "expected-metadata.json");
            test_metadata(data, Some("http://fakehost/test/"));
        }
    }
}
