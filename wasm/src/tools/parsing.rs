use std::{error::Error, vec};

use crate::{
    setting_schema::MainPluginSettings,
    tools::tokens::{HeadingLevel, MarkdownSection},
};

mod contents;
pub mod headings;

#[derive(Debug)]
struct ErrorInformation {
    reading_section_starting_line: usize,
}

pub fn get_sections(
    input: &str,
    settings: &MainPluginSettings,
) -> Result<Vec<MarkdownSection>, Box<dyn Error>> {
    use super::parsing::contents::{append_line_break, finish_current_content_section};
    use super::parsing::headings::{
        alternative_headings::get_alternative_heading_level,
        alternative_headings::validation::{
            validate_alternative_sub_heading, validate_alternative_top_heading,
        },
        get_top_heading_level,
        hash_headings::validation::{validate_sub_hash_heading, validate_top_hash_heading},
    };

    if input.is_empty() {
        return Ok(vec![]);
    }

    let mut sections = Vec::<MarkdownSection>::new();
    let input_lines = input.trim().split('\n').collect::<Vec<&str>>();

    // Get the top heading level and its hash literal.
    let mut top_heading_hash_literal = String::from("");
    let document_top_heading_level: Option<usize> = get_top_heading_level(&input_lines);
    if let Some(document_top_heading_level) = document_top_heading_level {
        top_heading_hash_literal = "#".repeat(document_top_heading_level);
    }

    let mut current_heading_level = 0;

    // Property section.
    let mut temp_properties = String::new();
    let mut is_reading_property_block = false;

    // Code block section.
    let mut temp_code_block = String::new();
    let mut is_reading_code_block = false;

    // Content section. (The rest part of the document.)
    // Everything goes into `MarkdownSection::Content` type,
    // unless it detects a markdown syntax that needs to be handled.
    let mut temp_content_section = String::new();
    let mut is_reading_content_section: bool = false;

    let mut error_information = ErrorInformation {
        reading_section_starting_line: 0,
    };

    for (index, &line) in input_lines.iter().enumerate() {
        // "is_reading_content_section" should be updated in the previous iteration.
        if line.is_empty() && !is_reading_content_section && !is_reading_code_block {
            continue;
        }
        is_reading_content_section = true;

        // Get alternative heading information.
        // Skip parsing the property section when it's detected.
        let previous_first_line: Option<&str> = if index > 0 {
            input_lines.get(index - 1).copied()
        } else {
            None
        };
        let previous_second_line: Option<&str> = if index > 1 {
            input_lines.get(index - 2).copied()
        } else {
            None
        };
        let next_line: Option<&str> = if index < input_lines.len() - 1 {
            input_lines.get(index + 1).copied()
        } else {
            None
        };
        let alternative_heading_level: Option<usize> =
            get_alternative_heading_level(&input_lines, index);

        // * Read Properties.
        if sections.is_empty() && (line == "---" || is_reading_property_block) {
            finish_current_content_section(
                &mut is_reading_content_section,
                &mut sections,
                &mut temp_content_section,
            );

            let is_first_property_line = temp_properties.is_empty()
                && previous_first_line.is_none()
                && previous_second_line.is_none()
                && next_line.is_some();
            if line == "---" {
                if is_first_property_line {
                    // Enter a property section.
                    error_information.reading_section_starting_line = index;
                    temp_properties.push_str(line);
                    is_reading_property_block = true;
                    continue;
                } else if is_reading_property_block {
                    // Exit a property section.
                    temp_properties.push('\n');
                    temp_properties.push_str(line);
                    is_reading_property_block = false;

                    sections.push(MarkdownSection::Property(temp_properties.clone()));
                    continue;
                }
            }

            // Keep reading properties.
            if is_reading_property_block {
                if !is_first_property_line {
                    temp_properties.push('\n');
                }
                temp_properties.push_str(line);
                continue;
            }
        }

        // * Read code blocks.
        if line.starts_with("```") || is_reading_code_block {
            finish_current_content_section(
                &mut is_reading_content_section,
                &mut sections,
                &mut temp_content_section,
            );

            if line.starts_with("```") {
                if !is_reading_code_block {
                    // Enter a code block.
                    error_information.reading_section_starting_line = index;
                    temp_code_block.push_str(line);
                    is_reading_code_block = true;
                    continue;
                } else {
                    // Exit a code block.
                    temp_code_block.push_str(format!("\n{}", line).as_str());
                    sections.push(MarkdownSection::Code(temp_code_block.clone()));

                    // Clear the temporary code block.
                    temp_code_block.clear();
                    is_reading_code_block = false;
                    continue;
                }
            }

            // Keep reading the code block.
            if is_reading_code_block {
                if !temp_code_block.is_empty() {
                    temp_code_block.push('\n');
                }
                temp_code_block.push_str(line);
                continue;
            }
        }

        // * Read hash headings.
        let only_contains_header_symbols = line.chars().all(|item| item == '#');
        if line.starts_with('#') && (line.contains("# ") || only_contains_header_symbols) {
            if let Some(document_top_heading_level) = document_top_heading_level {
                let is_top_heading = validate_top_hash_heading(line, &top_heading_hash_literal);

                if is_top_heading {
                    finish_current_content_section(
                        &mut is_reading_content_section,
                        &mut sections,
                        &mut temp_content_section,
                    );

                    sections.push(MarkdownSection::Heading(HeadingLevel::Top(
                        line.to_string(),
                    )));

                    current_heading_level = document_top_heading_level;
                    continue;
                } else {
                    let is_sub_heading =
                        validate_sub_hash_heading(line, only_contains_header_symbols);
                    let heading_level = line.chars().take_while(|&c| c == '#').count();

                    if is_sub_heading {
                        finish_current_content_section(
                            &mut is_reading_content_section,
                            &mut sections,
                            &mut temp_content_section,
                        );

                        if heading_level > current_heading_level {
                            sections.push(MarkdownSection::Heading(HeadingLevel::FirstSub(
                                line.to_string(),
                            )));
                        } else {
                            sections.push(MarkdownSection::Heading(HeadingLevel::Sub(
                                line.to_string(),
                            )));
                        }

                        current_heading_level = heading_level;
                        continue;
                    }
                }
            }
        }

        // * Read alternative headings.
        if let Some(alternative_heading_level) = alternative_heading_level {
            if is_reading_code_block || is_reading_property_block {
                continue;
            }

            let mut new_temp_content_section: Vec<String> = {
                let cloned_temp_content_section = temp_content_section.clone();
                cloned_temp_content_section
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            };
            new_temp_content_section.pop();
            temp_content_section.clear();
            temp_content_section = new_temp_content_section.join("\n");

            finish_current_content_section(
                &mut is_reading_content_section,
                &mut sections,
                &mut temp_content_section,
            );

            if let Some(document_top_heading_level) = document_top_heading_level {
                let is_top_heading = validate_alternative_top_heading(
                    &input_lines,
                    index,
                    document_top_heading_level,
                );
                let is_sub_heading = validate_alternative_sub_heading(
                    &input_lines,
                    index,
                    document_top_heading_level,
                );

                if let Some(previous_first_line) = previous_first_line {
                    if is_top_heading {
                        let mut pushing_value = previous_first_line.to_string();
                        pushing_value.push('\n');
                        pushing_value.push_str(line);

                        if sections.last()
                            == Some(&MarkdownSection::Content(previous_first_line.to_string()))
                        {
                            sections.pop();
                        }

                        sections.push(MarkdownSection::Heading(HeadingLevel::Top(pushing_value)));

                        current_heading_level = document_top_heading_level;
                        continue;
                    } else if is_sub_heading {
                        let mut pushing_value = previous_first_line.to_string();
                        pushing_value.push('\n');
                        pushing_value.push_str(line);

                        if sections.last()
                            == Some(&MarkdownSection::Content(previous_first_line.to_string()))
                        {
                            sections.pop();
                        }

                        if alternative_heading_level > current_heading_level {
                            sections.push(MarkdownSection::Heading(HeadingLevel::FirstSub(
                                pushing_value,
                            )));
                        } else {
                            sections
                                .push(MarkdownSection::Heading(HeadingLevel::Sub(pushing_value)));
                        }

                        current_heading_level = alternative_heading_level;
                        continue;
                    }
                }
            }
        }

        // * Read contents.
        if is_reading_content_section {
            error_information.reading_section_starting_line = index;
            append_line_break(&mut temp_content_section, line);
        }

        // Run this when it's the last line.
        if index == &input_lines.len() - 1 {
            finish_current_content_section(
                &mut is_reading_content_section,
                &mut sections,
                &mut temp_content_section,
            );
        }
    }

    // Return an error if the document is invalid.
    if is_reading_code_block || is_reading_property_block {
        let error_message =
            if let Some(true) = settings.other_options.show_more_detailed_error_messages {
                format!(
                    "Failed to parse the document.\n[Starting at: {}]",
                    error_information.reading_section_starting_line
                )
            } else {
                String::from("Failed to parse the document.")
            };

        return Err(error_message.into());
    }

    Ok(sections)
}
