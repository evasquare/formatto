use wasm_bindgen::prelude::*;

pub mod parsing_tools;
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

#[derive(Debug, PartialEq)]
pub enum HeadingLevel<'a> {
    Top(&'a str),
    FirstSub(&'a str),
    Sub(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum MarkdownSection<'a> {
    Property,
    Heading(HeadingLevel<'a>),
    Content,
    Unknown(&'a str),
}

#[wasm_bindgen]
pub fn format_document(input: &str) -> String {
    use parsing_tools::divide_top_headings;

    let top_heading_sections = divide_top_headings(input);
    console_log!("top_heading_sections: {:#?}", top_heading_sections);

    input.to_string()
}
