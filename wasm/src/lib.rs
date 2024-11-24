use std::error::Error;
use utils::Preferences;
use wasm_bindgen::prelude::*;

mod option_schema;
mod tools;
mod utils;

#[cfg(test)]
mod testing;

#[allow(dead_code)]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn log(s: &str) {
    println!("{}", s);
}
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn error(s: &str) {
    eprintln!("{}", s);
}

mod macro_rules {
    #[macro_export]
    macro_rules! console_log {
        ($($arg:tt)*) => ($crate::log(&format!($($arg)*)));
    }
    #[macro_export]
    macro_rules! console_error {
        ($($arg:tt)*) => ($crate::error(&format!($($arg)*)));
    }
}

#[wasm_bindgen]
pub struct FormattedDocument {
    document: String,
    editor_position: EditorPosition,
}
#[wasm_bindgen]
impl FormattedDocument {
    #[wasm_bindgen(getter)]
    pub fn document(&self) -> String {
        self.document.clone()
    }
    #[wasm_bindgen(getter, js_name = editorPosition)]
    pub fn editor_position(&self) -> EditorPosition {
        self.editor_position.clone()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct EditorPosition {
    line: usize,
    ch: usize,
}
#[wasm_bindgen]
impl EditorPosition {
    #[wasm_bindgen(constructor)]
    pub fn new(line: usize, ch: usize) -> Self {
        EditorPosition { line, ch }
    }

    #[wasm_bindgen(getter)]
    pub fn line(&self) -> usize {
        self.line
    }

    #[wasm_bindgen(getter)]
    pub fn ch(&self) -> usize {
        self.ch
    }
}

#[wasm_bindgen]
/// This function will be called from the TypeScript side.
pub fn format_document(
    input: &str,
    original_cursor_position: EditorPosition,
    js_options: JsValue,
    js_locales: JsValue,
) -> FormattedDocument {
    use utils::{read_js_value, read_options};

    utils::set_panic_hook();

    let options = match read_options(js_options) {
        Ok(options) => options,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    };

    let locales = match read_js_value(js_locales) {
        Ok(locales) => locales,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    };

    if input.is_empty() {
        return FormattedDocument {
            document: "".to_string(),
            editor_position: EditorPosition { line: 0, ch: 0 },
        };
    }

    let preferences = Preferences { options, locales };

    // Return output to the TypeScript side or throw an error.
    let formetted_document = match parse_input(input, &original_cursor_position, &preferences) {
        Ok(sections) => sections,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    };

    FormattedDocument {
        document: formetted_document.0,
        editor_position: EditorPosition {
            line: formetted_document.1,
            ch: original_cursor_position.ch,
        },
    }
}

/// Parses an input and returns a formatted string.
fn parse_input(
    input: &str,
    original_cursor_position: &EditorPosition,
    preferences: &Preferences,
) -> Result<(String, usize), Box<dyn Error>> {
    let sections = tools::parsing::get_sections(input, original_cursor_position, preferences)?;
    let output = tools::formatting::get_formatted_string(sections.0, sections.1, preferences)?;

    Ok((output.0, output.1))
}
