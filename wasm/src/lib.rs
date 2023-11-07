use wasm_bindgen::prelude::*;

mod format_tools;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => ($crate::log(&format!($($arg)*)));
}

#[wasm_bindgen]
pub fn status() -> bool {
    true
}

enum HeadingLevel {
    Top,
    FirstSub,
    Sub,
}

enum MarkdownComponent {
    Property,
    Heading(HeadingLevel),
    Content,
}

#[wasm_bindgen]
pub fn format_document(input: &str) -> String {
    use format_tools::divide_top_headings;

    let top_heading_sections: Vec<Vec<&str>> = divide_top_headings(input);
    console_log!("{:#?}", top_heading_sections);

    input.to_string()
}
