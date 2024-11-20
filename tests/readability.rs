use std::fs;

mod common;

use common::test_readability;

#[test]
fn test_engadget() {
    test_readability(
        "test-pages/controversial/engadget/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_hukumusume() {
    //TODO: ???
    test_readability(
        "test-pages/issues/hukumusume/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_la_nacion() {
    test_readability(
        "test-pages/controversial/la-nacion/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_wikipedia_3() {
    test_readability(
        "test-pages/controversial/wikipedia-3/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_citylab_1() {
    test_readability(
        "test-pages/issues/citylab-1/",
        Some("http://fakehost/test/"),
    );
}

#[test]
fn test_dev418() {
    test_readability("test-pages/issues/dev418/", Some("http://fakehost/test/"));
}



#[test]
fn test_table_style_attributes() {
    test_readability(
        "test-pages/issues/table-style-attributes/",
        Some("http://fakehost/test/"),
    )
}



#[test]
fn table_test_readability() {
    let paths = fs::read_dir("./test-pages/readability").unwrap();

    for p in paths {
        println!("{}", p.as_ref().unwrap().path().display());
        test_readability(p.unwrap().path(), Some("http://fakehost/test/"));
    }
}
