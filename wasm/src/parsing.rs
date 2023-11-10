use crate::console_log;

use super::types::setting_types::MainPluginSettings;
use std::error::Error;

pub mod parsing_tools;

pub fn parse_input(input: &str, settings: MainPluginSettings) -> Result<String, Box<dyn Error>> {
    let sections = parsing_tools::get_sections(input);
    console_log!("sections: {:#?}", sections);
    let output = parsing_tools::get_formatted_string(sections, &settings)?;

    Ok(output)
}
