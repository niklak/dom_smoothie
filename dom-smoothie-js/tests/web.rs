//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;


//wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_parse() {
    let contents = include_str!("../../test-pages/rustwiki_2024.html");
    let res = dom_smoothie_js::parse(contents);
    assert!(res.is_ok());
}

#[wasm_bindgen_test]
fn test_parse_constructor() {
    let contents = include_str!("../../test-pages/rustwiki_2024.html");

    let mut ra =
        dom_smoothie_js::Readability::new(contents.to_string(), None, JsValue::null()).unwrap();

    let article = ra.parse();
    assert!(article.is_ok());
}
