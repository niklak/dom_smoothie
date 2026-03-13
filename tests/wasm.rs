#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

#[macro_use]
mod common;

use common::{test_metadata, test_readability, TestData};

#[wasm_bindgen_test]
fn test_readability_wikipedia_2() {
    let data = include_test_data!(
        "test-pages/ok/wikipedia-2/",
        "../test-pages/ok/wikipedia-2/source.html",
        "../test-pages/ok/wikipedia-2/expected.html"
    );
    test_readability(data);
}

#[wasm_bindgen_test]
fn test_metadata_wikipedia_2() {
    let data = include_test_data!(
        "test-pages/ok/wikipedia-2/",
        "../test-pages/ok/wikipedia-2/source.html",
        "../test-pages/ok/wikipedia-2/expected-metadata.json"
    );

    test_metadata(data, Some("http://fakehost/test/"));
}
