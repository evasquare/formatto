use setting_types::MainPluginSettings;
use std::error::Error;
use wasm_bindgen::prelude::*;

pub mod parsing_tools;
pub mod token_types;

mod macro_rules;
mod setting_types;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
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

    match parsing_tools::parse_input(input, settings) {
        Ok(sections) => sections,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    }
}
