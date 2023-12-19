use std::error::Error;
use wasm_bindgen::prelude::*;

use crate::setting_schema::MainPluginSettings;

mod setting_schema;
mod testing;
mod tools;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
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

#[wasm_bindgen(js_name = formatDocument)]
pub fn format_document(
    input: &str,
    js_settings: JsValue,
    original_cursor_position: EditorPosition,
) -> FormattedDocument {
    let settings = match read_settings(js_settings) {
        Ok(settings) => settings,
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

    let extra_information = ExtraInformation {
        settings: &settings,
        original_cursor_position,
    };

    // Return value to the TypeScript side or throw an error.
    let formatted_document = match parse_input(input, extra_information) {
        Ok(sections) => sections,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    };

    FormattedDocument {
        document: formatted_document,
        editor_position: EditorPosition { line: 0, ch: 0 },
    }
}

fn read_settings(settings: JsValue) -> Result<MainPluginSettings, Box<dyn Error>> {
    Ok(serde_wasm_bindgen::from_value(settings)?)
}

#[derive(Debug)]
struct ExtraInformation<'a> {
    settings: &'a MainPluginSettings,
    original_cursor_position: EditorPosition,
}
fn parse_input(input: &str, extra_information: ExtraInformation) -> Result<String, Box<dyn Error>> {
    let sections = tools::parsing::get_sections(
        input,
        ExtraInformation {
            settings: extra_information.settings,
            original_cursor_position: extra_information.original_cursor_position,
        },
    )?;
    let output = tools::formatting::get_formatted_string(sections, extra_information.settings)?;

    Ok(output)
}
