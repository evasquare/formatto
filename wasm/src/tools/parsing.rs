use std::{error::Error, vec};

use crate::console_error;
use crate::tools::tokens::{HeadingLevel, MarkdownSection};

pub fn get_sections(input: &str) -> Result<Vec<MarkdownSection>, Box<dyn Error>> {
    if input.is_empty() {
        return Ok(vec![]);
    }

    let mut sections = Vec::<MarkdownSection>::new();
    let input_lines = input.trim().split('\n').collect::<Vec<&str>>();

    // Get the top heading level and its hash literal.
    let mut top_heading_hash_literal = String::from("");
    let top_heading_level: Option<usize> = get_top_heading_level(&input_lines);
    if let Some(top_heading_level) = top_heading_level {
        top_heading_hash_literal = "#".repeat(top_heading_level);
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

    for (index, &line) in input_lines.iter().enumerate() {
        // "is_reading_content_section" should be updated in the previous iteration.
        if line.is_empty() && !is_reading_content_section && !is_reading_code_block {
            continue;
        }
        is_reading_content_section = true;

        // * Read Properties.
        if sections.is_empty() && (line == "---" || is_reading_property_block) {
            finish_current_content_section(
                &mut is_reading_content_section,
                &mut sections,
                &mut temp_content_section,
            );

            let is_first_property_line = temp_properties.is_empty();
            if line == "---" {
                if is_first_property_line {
                    // Enter a property section.
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

        // * Read headings.
        let only_contains_header_symbols = line.chars().all(|item| item == '#');
        if line.starts_with('#') && (line.contains("# ") || only_contains_header_symbols) {
            if let Some(top_heading_level) = top_heading_level {
                let is_top_heading = check_top_hash_heading(line, &top_heading_hash_literal);

                if is_top_heading {
                    finish_current_content_section(
                        &mut is_reading_content_section,
                        &mut sections,
                        &mut temp_content_section,
                    );

                    sections.push(MarkdownSection::Heading(HeadingLevel::Top(
                        line.to_string(),
                    )));

                    current_heading_level = top_heading_level;
                } else {
                    let is_sub_heading = check_sub_hash_heading(line, only_contains_header_symbols);
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
                    }
                }
            }
        }

        // * Read contents.
        if is_reading_content_section {
            append_string_with_line_break(&mut temp_content_section, line);
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
        return Err(String::from("Failed to parse the document.").into());
    }

    Ok(sections)
}

// Functions for parsing hash symbol headings.
fn check_top_hash_heading(line: &str, top_heading_hash_literal: &str) -> bool {
    line.starts_with(top_heading_hash_literal)
        && !line.starts_with(format!("{}#", top_heading_hash_literal).as_str())
}
fn check_sub_hash_heading(line: &str, only_contains_header_symbols: bool) -> bool {
    line.contains("# ") || only_contains_header_symbols
}

// Functions for reading "content" sections.
/// Append a line with a line break.
fn append_string_with_line_break(string: &mut String, line: &str) {
    // Break lines unless it's the first line.
    if !string.is_empty() {
        string.push('\n');
    }
    string.push_str(line);
}
/// Finish reading the current "content" section and push it to the "sections" vector.
fn finish_current_content_section(
    is_reading_content_section: &mut bool,
    sections: &mut Vec<MarkdownSection>,
    temp_content_section: &mut String,
) {
    *is_reading_content_section = false;

    // Check if "content" is empty.
    // Because this function is also called with empty values.
    if temp_content_section.is_empty() {
        return;
    }

    sections.push(MarkdownSection::Content(
        temp_content_section.trim_end().to_string(),
    ));
    temp_content_section.clear();
}

/// Receive lines of a markdown document and return the top heading level.
pub fn get_top_heading_level(input_lines: &[&str]) -> Option<usize> {
    let mut top_heading_level: usize = usize::MAX;
    let mut is_reading_code_block = false;

    for line in input_lines {
        // Skip code blocks.
        if line.starts_with("```") {
            is_reading_code_block = !is_reading_code_block;
        }
        if is_reading_code_block {
            continue;
        }

        let valid_hash_heading =
            line.starts_with('#') && (line.contains("# ") || line.chars().all(|char| char == '#'));

        if valid_hash_heading {
            let heading_level = line.chars().take_while(|&c| c == '#').count();
            if heading_level < top_heading_level {
                top_heading_level = heading_level;
            }
        }
    }

    if top_heading_level == usize::MAX {
        return None;
    }

    Some(top_heading_level)
}

/// Parse a usize value from a &str type argument.
/// Also return an `Error` to handle exceptions.
pub fn parse_str_to_usize(input: &Option<String>) -> Result<usize, Box<dyn Error>> {
    if let Some(input) = input {
        if input.is_empty() {
            return Err(String::from(
                "Failed to read settings. Please make sure there is no option with an empty value.",
            )
            .into());
        }
    }

    match input {
        Some(input) => match input.parse::<usize>() {
            Ok(num) => Ok(num),
            Err(err) => {
                console_error!("{}", err);
                Err(String::from(
                    "Failed to read settings. Some of them are possibly not positive number values.",
                )
                .into())
            }
        },
        None => Err(String::from("Failed to access setting properties.").into()),
    }
}
