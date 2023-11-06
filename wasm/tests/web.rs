//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use std::assert;

use formatto_wasm::format_document;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert!(format_document("test input") != "");
}
