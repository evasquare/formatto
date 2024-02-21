use serde_json::Value;

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

pub enum LocaleCategory {
    Parsing,
    Formatting,
}

pub fn get_locale_string(locales: &Value, category: LocaleCategory, key: &str) -> String {
    match category {
        LocaleCategory::Parsing => {
            if let Some(msg) = locales["parsing"][key].as_str() {
                return String::from(msg);
            }
        }
        LocaleCategory::Formatting => {
            if let Some(msg) = locales["formatting"][key].as_str() {
                return String::from(msg);
            }
        }
    }
    String::from(key)
}
