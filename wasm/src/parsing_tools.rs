use std::error::Error;

use crate::console_error;
use crate::setting_types::MainPluginSettings;
use crate::token_types::{HeadingLevel, MarkdownSection};

// TODO: Read other markdown sections too.

pub fn get_section_vec(input: &str) -> Vec<MarkdownSection> {
    let input_lines: Vec<&str> = input.trim().split('\n').collect::<Vec<&str>>();

    let top_heading_level = input_lines[0].chars().take_while(|&c| c == '#').count();
    let top_heading_literal = "#".repeat(top_heading_level);

    let mut sections = Vec::<MarkdownSection>::new();

    let mut code_block = String::new();
    let mut is_reading_code_block = false;

    for line in input_lines {
        // * Parse code blocks.
        {
            // Entering and exiting a code block.
            if line.starts_with("```") && !is_reading_code_block {
                code_block.push_str(line);

                is_reading_code_block = true;
                continue;
            }
            if line.starts_with("```") && is_reading_code_block {
                code_block.push_str(format!("\n{}", line).as_str());
                sections.push(MarkdownSection::Code(code_block.clone()));

                // Clear the temporary code block.
                code_block.clear();

                is_reading_code_block = false;
                continue;
            }

            // Keep reading the code block.
            if is_reading_code_block {
                if !code_block.is_empty() {
                    code_block.push('\n');
                }

                code_block.push_str(line);
                continue;
            }
        }

        // * Parse headings.
        {
            let is_top_heading = line.starts_with(&top_heading_literal)
                && !line.starts_with(format!("{}#", top_heading_literal).as_str());

            // TODO: Parse sub-headings.

            if !line.is_empty() {
                sections.push(match is_top_heading {
                    true => MarkdownSection::Heading(HeadingLevel::Top(line.to_string())),
                    false => MarkdownSection::Unknown(line.to_string()),
                });
            }
        }
    }

    sections
}

pub fn insert_line_breaks(content: &str, before: usize, after: usize) -> String {
    let line_breaks_before = "\n".repeat(before);
    let line_breaks_after = "\n".repeat(after);

    format!("{}{}{}", line_breaks_before, content, line_breaks_after)
}

fn parse_str_to_usize(input: &Option<String>) -> Result<usize, Box<dyn Error>> {
    match input {
        Some(input) => match input.parse::<usize>() {
            Ok(num) => Ok(num),
            Err(err) => {
                console_error!("{}", err);
                Err(String::from(
                    "Failed to read settings. Some of them are possibly not number values.",
                )
                .into())
            }
        },
        None => Err(String::from("Failed to access setting properties.").into()),
    }
}

fn get_formatted_string(
    sections: Vec<MarkdownSection>,
    settings: &MainPluginSettings,
) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();

    for section in sections {
        match section {
            MarkdownSection::Heading(heading_level) => match heading_level {
                HeadingLevel::Top(content) => {
                    if output.is_empty() || output.split('\n').count() == 0 {
                        output.push_str(&content);
                        continue;
                    }

                    output.push_str(&insert_line_breaks(
                        &content,
                        parse_str_to_usize(&settings.heading_gaps.top_level_headings)?,
                        0,
                    ));
                }
                HeadingLevel::FirstSub(content) => {
                    let formatted = insert_line_breaks(
                        &content,
                        parse_str_to_usize(&settings.heading_gaps.first_sub_heading)?,
                        0,
                    );
                    output.push_str(&formatted);
                }
                HeadingLevel::Sub(content) => {
                    output.push_str(&insert_line_breaks(
                        &content,
                        parse_str_to_usize(&settings.heading_gaps.sub_headings)?,
                        0,
                    ));
                }
            },
            MarkdownSection::Code(content) => output.push_str(&insert_line_breaks(&content, 1, 0)),
            MarkdownSection::Unknown(content) => {
                output.push_str(&insert_line_breaks(&content, 1, 0));
            }
            MarkdownSection::Property(content) => {
                output.push_str(&insert_line_breaks(
                    &content,
                    0,
                    parse_str_to_usize(&settings.property_gaps.after_properties)?,
                ));
            }
            MarkdownSection::Content(content) => {
                output.push_str(&insert_line_breaks(
                    &content,
                    parse_str_to_usize(&settings.other_gaps.contents_after_headings)?,
                    0,
                ));
            }
        }
    }

    Ok(output)
}

pub fn parse_input(input: &str, settings: MainPluginSettings) -> Result<String, Box<dyn Error>> {
    let sections = get_section_vec(input);
    let output = get_formatted_string(sections, &settings)?;

    Ok(output)
}
