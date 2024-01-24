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
pub fn status() -> bool {
    true
}

#[wasm_bindgen]
/// This function is called from the TypeScript side.
pub fn format_document(input: &str, js_settings: JsValue) -> String {
    utils::set_panic_hook();

    let settings = match read_settings(js_settings) {
        Ok(settings) => settings,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    };

    if input.is_empty() {
        return input.to_string();
    }

    // Return value to the TypeScript side or throw an error.
    match parse_input(input, settings) {
        Ok(sections) => sections,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    }
}

fn read_settings(settings: JsValue) -> Result<MainPluginSettings, Box<dyn Error>> {
    Ok(serde_wasm_bindgen::from_value(settings)?)
}

fn parse_input(input: &str, settings: MainPluginSettings) -> Result<String, Box<dyn Error>> {
    let sections = tools::parsing::get_sections(input)?;
    let output = tools::formatting::get_formatted_string(sections, &settings)?;

    Ok(output)
}
