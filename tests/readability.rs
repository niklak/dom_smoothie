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
