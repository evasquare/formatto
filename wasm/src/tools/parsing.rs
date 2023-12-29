use std::{error::Error, vec};

use crate::console_error;
use crate::tools::tokens::{HeadingLevel, MarkdownSection};

pub fn get_sections(input: &str) -> Result<Vec<MarkdownSection>, Box<dyn Error>> {
    if input.is_empty() {
        return Ok(vec![]);
    }

    let mut sections = Vec::<MarkdownSection>::new();
    let input_lines = input.trim().split('\n');
    let input_lines_vec = input_lines.clone().collect::<Vec<&str>>();

    let mut md_top_heading_level: usize = 0;
    let mut md_top_heading_literal = String::from("");

    let contains_heading = input.split('\n').any(|line| {
        !line.is_empty() && (line.contains("# ") || line.chars().all(|item| item == '#'))
    });
    if contains_heading {
        md_top_heading_level = get_top_heading_level(&input_lines_vec);
        md_top_heading_literal = "#".repeat(md_top_heading_level);
    }

    let mut md_properties = String::new();
    let mut is_reading_md_properties = false;
    let mut md_code_block = String::new();
    let mut is_reading_md_code_block = false;

    let mut is_reading_md_content = false;
    let mut currently_reading_heading_level = 0;

    // Everything goes into `MarkdownSection::Content` type,
    // unless it detects a markdown syntax that needs to be handled.
    let mut md_content = String::new();

    for (index, line) in input_lines.enumerate() {
        // "is_reading_md_content" gets updated in the previous iteration.
        if line.is_empty() && !is_reading_md_content && !is_reading_md_code_block {
            continue;
        }
        is_reading_md_content = true;

        // * Read Properties.
        if sections.is_empty() && (line == "---" || is_reading_md_properties) {
            is_reading_md_content = false;
            push_content_section(&mut sections, &mut md_content);

            let is_first_property_line = md_properties.is_empty();

            // Enter and exit point of a property section.
            if line == "---" {
                if is_first_property_line {
                    md_properties.push_str(line);
                    is_reading_md_properties = true;
                    continue;
                } else if is_reading_md_properties {
                    md_properties.push('\n');
                    md_properties.push_str(line);
                    is_reading_md_properties = false;

                    sections.push(MarkdownSection::Property(md_properties.clone()));
                    continue;
                }
            }

            // Keep reading properties.
            if is_reading_md_properties {
                if !is_first_property_line {
                    md_properties.push('\n');
                }
                md_properties.push_str(line);
                continue;
            }
        }

        // * Read code blocks.
        if line.starts_with("```") || is_reading_md_code_block {
            is_reading_md_content = false;
            push_content_section(&mut sections, &mut md_content);

            // Enter and exit point of a code block.
            if line.starts_with("```") {
                if !is_reading_md_code_block {
                    md_code_block.push_str(line);
                    is_reading_md_code_block = true;
                    continue;
                } else {
                    md_code_block.push_str(format!("\n{}", line).as_str());
                    sections.push(MarkdownSection::Code(md_code_block.clone()));

                    // Clear the temporary code block.
                    md_code_block.clear();
                    is_reading_md_code_block = false;
                    continue;
                }
            }

            // Keep reading the code block.
            if is_reading_md_code_block {
                if !md_code_block.is_empty() {
                    md_code_block.push('\n');
                }
                md_code_block.push_str(line);
                continue;
            }
        }

        // * Read headings.
        let only_contains_header_symbols = line.chars().all(|item| item == '#');
        if line.starts_with('#') && (line.contains("# ") || only_contains_header_symbols) {
            let is_top_heading = line.starts_with(&md_top_heading_literal)
                && !line.starts_with(format!("{}#", md_top_heading_literal).as_str());

            if is_top_heading {
                is_reading_md_content = false;
                push_content_section(&mut sections, &mut md_content);

                sections.push(MarkdownSection::Heading(HeadingLevel::Top(
                    line.to_string(),
                )));

                currently_reading_heading_level = md_top_heading_level;
            } else {
                let is_sub_heading = line.contains("# ") || only_contains_header_symbols;
                let heading_level = line.chars().take_while(|&c| c == '#').count();

                if is_sub_heading {
                    is_reading_md_content = false;
                    push_content_section(&mut sections, &mut md_content);

                    if heading_level > currently_reading_heading_level {
                        sections.push(MarkdownSection::Heading(HeadingLevel::FirstSub(
                            line.to_string(),
                        )));
                    } else {
                        sections.push(MarkdownSection::Heading(HeadingLevel::Sub(
                            line.to_string(),
                        )));
                    }

                    currently_reading_heading_level = heading_level;
                }
            }
        }

        // * Read contents.
        if is_reading_md_content {
            append_string_with_line_breaks(&mut md_content, line);
        }

        // Run this when it's the last line.
        if index == input_lines_vec.len() - 1 {
            push_content_section(&mut sections, &mut md_content);
        }
    }

    // Return an error if the document is invalid.
    if is_reading_md_code_block || is_reading_md_properties {
        return Err(String::from("Failed to parse the document.").into());
    }

    Ok(sections)
}

// Functions used for reading "Content" sections.
fn append_string_with_line_breaks(string: &mut String, line: &str) {
    // Break lines unless it's the first line.
    if !string.is_empty() {
        string.push('\n');
    }
    string.push_str(line);
}
fn push_content_section(sections: &mut Vec<MarkdownSection>, content: &mut String) {
    // Check if "content" is empty.
    // Because this function is also called with empty values.
    if content.is_empty() {
        return;
    }

    sections.push(MarkdownSection::Content(content.trim_end().to_string()));
    content.clear();
}

/// Receive lines of a markdown document and return the top heading level.
pub fn get_top_heading_level(input_lines: &[&str]) -> usize {
    let mut top_heading_level: usize = usize::MAX;
    let mut is_reading_md_code_block = false;

    for line in input_lines {
        // Skip code blocks.
        if line.starts_with("```") {
            is_reading_md_code_block = !is_reading_md_code_block;
        }
        if is_reading_md_code_block {
            continue;
        }

        let current_line_level = line.chars().take_while(|&c| c == '#').count();
        if line.starts_with('#') && top_heading_level > current_line_level {
            top_heading_level = current_line_level;
        }
    }

    top_heading_level
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
