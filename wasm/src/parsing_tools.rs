use std::error::Error;

use crate::console_error;
use crate::setting_types::MainPluginSettings;
use crate::token_types::{HeadingLevel, MarkdownSection};

pub fn parse_input(input: &str, settings: MainPluginSettings) -> Result<String, Box<dyn Error>> {
    let sections = get_section_vec(input);
    let output = get_formatted_string(sections, &settings)?;

    Ok(output)
}

pub fn get_section_vec(input: &str) -> Vec<MarkdownSection> {
    let mut sections = Vec::<MarkdownSection>::new();
    let input_lines: Vec<&str> = input.trim().split('\n').collect::<Vec<&str>>();

    let mut properties = String::new();
    let mut is_reading_properties = false;

    let mut code_block = String::new();
    let mut is_reading_code_block = false;

    let top_heading_level = get_top_heading_level(&input_lines);
    let top_heading_literal = "#".repeat(top_heading_level);

    for line in input_lines {
        // * Parse Properties.
        if sections.is_empty() && (line == "---" || is_reading_properties) {
            let is_empty = sections.is_empty() && properties.is_empty();

            // Entering and exiting properties section.
            if line == "---" && is_empty {
                properties.push_str(line);
                is_reading_properties = true;
                continue;
            } else if line == "---" && is_reading_properties {
                properties.push('\n');
                properties.push_str(line);
                is_reading_properties = false;

                sections.push(MarkdownSection::Property(properties.clone()));
                continue;
            }

            // Keep reading properties.
            if is_reading_properties {
                if is_empty {
                    properties.push_str(line);
                    continue;
                }

                properties.push('\n');
                properties.push_str(line);
                continue;
            }
        }

        // * Parse code blocks.
        if line.starts_with("```") || is_reading_code_block {
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

            if is_top_heading && !line.is_empty() {
                sections.push(MarkdownSection::Heading(HeadingLevel::Top(
                    line.to_string(),
                )));
            }

            if !is_top_heading {
                let filtered_string = line
                    .chars()
                    .take_while(|&c| c == '#' || c == ' ')
                    .collect::<Vec<char>>();

                let is_sub_heading = filtered_string.last().map_or(false, |last_char| {
                    *last_char == ' ' && filtered_string.len() > 1
                });

                if is_sub_heading && !line.is_empty() {
                    sections.push(MarkdownSection::Heading(HeadingLevel::Sub(
                        line.to_string(),
                    )));
                }
            }

            continue;
        }

        // TODO: Parse contents.
    }

    sections
}

fn get_formatted_string(
    sections: Vec<MarkdownSection>,
    settings: &MainPluginSettings,
) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();
    let mut right_after_properties = false;
    let after_properties_gap = parse_str_to_usize(&settings.other_gaps.after_properties)? + 1;

    for section in sections {
        match section {
            MarkdownSection::Property(content) => {
                output.push_str(&content);
                right_after_properties = true;
            }
            MarkdownSection::Heading(heading_level) => match heading_level {
                HeadingLevel::Top(content) => {
                    output.push_str(&insert_line_breaks(
                        &content,
                        if right_after_properties {
                            after_properties_gap
                        } else {
                            parse_str_to_usize(&settings.heading_gaps.top_level_headings)?
                        },
                        0,
                    ));
                }
                HeadingLevel::FirstSub(content) => {
                    let formatted = insert_line_breaks(
                        &content,
                        if right_after_properties {
                            after_properties_gap
                        } else {
                            parse_str_to_usize(&settings.heading_gaps.first_sub_heading)?
                        },
                        0,
                    );
                    output.push_str(&formatted);
                }
                HeadingLevel::Sub(content) => {
                    output.push_str(&insert_line_breaks(
                        &content,
                        if right_after_properties {
                            after_properties_gap
                        } else {
                            parse_str_to_usize(&settings.heading_gaps.sub_headings)?
                        },
                        0,
                    ));
                }
            },
            MarkdownSection::Content(content) => {
                output.push_str(&insert_line_breaks(
                    &content,
                    if right_after_properties {
                        after_properties_gap
                    } else {
                        parse_str_to_usize(&settings.other_gaps.contents_after_headings)?
                    },
                    0,
                ));
            }
            MarkdownSection::Code(content) => output.push_str(&insert_line_breaks(
                &content,
                if right_after_properties {
                    after_properties_gap
                } else {
                    parse_str_to_usize(&settings.other_gaps.before_code_blocks)?
                },
                0,
            )),
            MarkdownSection::Unknown(content) => {
                output.push_str(&insert_line_breaks(&content, 1, 0));
            }
        }
    }

    Ok(output)
}

pub fn get_top_heading_level(input_lines: &[&str]) -> usize {
    let mut top_heading_level: usize = usize::MAX;

    for line in input_lines {
        let current_line_level = line.chars().take_while(|&c| c == '#').count();

        if line.starts_with('#') && top_heading_level > current_line_level {
            top_heading_level = current_line_level;
        }
    }

    top_heading_level
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
