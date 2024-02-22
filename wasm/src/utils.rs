use serde_json::Value;
use std::error::Error;
use wasm_bindgen::JsValue;

use crate::setting_schema::PluginSettings;

// To use the feature, use the following argument.
// "--features development"
// ex) "wasm-pack build --target web --features development"

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(all(feature = "development", feature = "console_error_panic_hook"))]
    console_error_panic_hook::set_once();
}

/// Struct for data from the TypeScript side.
#[derive(Debug)]
pub struct Preferences {
    pub settings: PluginSettings,
    pub locales: Value,
}

/// Reads the Obsidian plugin's setting data.
pub fn read_settings<T: serde::de::DeserializeOwned>(input: JsValue) -> Result<T, Box<dyn Error>> {
    Ok(serde_wasm_bindgen::from_value(input)?)
}

/// Reads a JSON string value.
pub fn read_js_value(js_value: JsValue) -> Result<Value, Box<dyn Error>> {
    if let Some(js_value) = &js_value.as_string() {
        Ok(serde_json::from_str(js_value)?)
    } else {
        Err("Failed to read locale file.".into())
    }
}

/// The category of a message.
pub enum LocaleCategory {
    Parsing,
    Formatting,
}

/// Returns a message in the user's language.
/// If it fails to get the value, it returns the key.
pub fn get_locale_string(locales: &Value, category: LocaleCategory, key: &str) -> String {
    match category {
        LocaleCategory::Parsing => {
            if let Some(message) = locales["parsing"][key].as_str() {
                if !message.is_empty() {
                    return String::from(message);
                }
            }
        }
        LocaleCategory::Formatting => {
            if let Some(message) = locales["formatting"][key].as_str() {
                if !message.is_empty() {
                    return String::from(message);
                }
            }
        }
    }
    String::from(key)
}
