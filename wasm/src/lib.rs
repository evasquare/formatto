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
pub enum HeadingLevel {
    Top(String),
    FirstSub(String),
    Sub(String),
}

#[derive(Debug, PartialEq)]
pub enum MarkdownSection {
    Property,
    Heading(HeadingLevel),
    Content,
    Code(String),
    Unknown(String),
}

#[wasm_bindgen]
pub fn format_document(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }

    match parsing_tools::parse_input(input) {
        Ok(sections) => sections,
        Err(e) => {
            let error_message = e.to_string();

            // Display an error message in Obsidian.
            wasm_bindgen::throw_str(&error_message);
        }
    }
}
