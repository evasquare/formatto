use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn status() -> bool {
    true
}

#[wasm_bindgen]
pub fn format_document(input: &str) -> String {
    log(input);

    // TODO: implement the formatting feature here.
    let formatted_value = format!("# TEST\n{}", input);
    formatted_value.to_string()
}
