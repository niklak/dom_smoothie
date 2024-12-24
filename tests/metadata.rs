use std::fs;

mod common;

use common::test_metadata;


#[test]
fn test_metadata_simplyfound_1() {
    test_metadata("./test-pages/readability/simplyfound-1", Some("http://fakehost/test/"));
}

#[test]
fn table_test_metadata() {
    let paths = fs::read_dir("./test-pages/readability").unwrap();
    for p in paths {
        let pp = p.unwrap().path();
        println!("{:?}", &pp);
        test_metadata(pp, Some("http://fakehost/test/"));
    }
}
