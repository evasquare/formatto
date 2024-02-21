use std::error::Error;
use wasm_bindgen::prelude::*;

use crate::setting_schema::PluginSettings;

mod setting_schema;
mod tools;
mod utils;

#[cfg(test)]
mod testing;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log(s: &str) {
    println!("{}", s);
}
#[cfg(not(target_arch = "wasm32"))]
pub fn error(s: &str) {
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
/// This function is called from the TypeScript side.
pub fn format_document(input: &str, js_settings: JsValue, js_locales: JsValue) -> String {
    utils::set_panic_hook();

    let settings: PluginSettings = match read_settings(js_settings) {
        Ok(settings) => settings,
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
        return input.to_string();
    }

    // Return value to the TypeScript side or throw an error.
    match parse_input(input, settings, &locales) {
        Ok(sections) => sections,
        Err(e) => {
            let error_message = e.to_string();
            wasm_bindgen::throw_str(&error_message);
        }
    }
}

fn read_settings<T: serde::de::DeserializeOwned>(input: JsValue) -> Result<T, Box<dyn Error>> {
    Ok(serde_wasm_bindgen::from_value(input)?)
}

use serde_json::Value;
fn read_js_value(js_value: JsValue) -> Result<Value, Box<dyn Error>> {
    if let Some(a) = &js_value.as_string() {
        Ok(serde_json::from_str(a)?)
    } else {
        Err("Failed to read locale file.".into())
    }
}

fn parse_input(
    input: &str,
    settings: PluginSettings,
    locales: &Value,
) -> Result<String, Box<dyn Error>> {
    let sections = tools::parsing::get_sections(input, &settings, locales)?;
    let output = tools::formatting::get_formatted_string(sections, &settings, locales)?;

    Ok(output)
}
