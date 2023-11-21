use std::error::Error;

use super::types::setting_types::MainPluginSettings;
pub mod parsing_tools;

pub fn parse_input(input: &str, settings: MainPluginSettings) -> Result<String, Box<dyn Error>> {
    let sections = parsing_tools::get_sections(input);
    let output = parsing_tools::get_formatted_string(sections, &settings)?;

    Ok(output)
}
