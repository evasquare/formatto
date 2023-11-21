use std::error::Error;
use types::setting_types::MainPluginSettings;
use wasm_bindgen::prelude::*;

mod parsing;
mod testing;
mod utils;

mod types {
    pub mod setting_types;
    pub mod token_types;
}

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

fn read_settings(settings: JsValue) -> Result<MainPluginSettings, Box<dyn Error>> {
    Ok(serde_wasm_bindgen::from_value(settings)?)
}

#[wasm_bindgen]
pub fn format_document(input: &str, js_settings: JsValue) -> String {
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
    match parsing::parse_input(input, settings) {
        Ok(sections) => sections,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    }
}
