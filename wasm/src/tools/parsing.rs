use std::error::Error;

use crate::{
    console_log,
    tools::tokens::{HeadingLevel, MarkdownSection},
    EditorPosition, Preferences,
};

mod contents;
pub mod headings;

#[derive(Debug)]
struct ErrorInformation {
    reading_section_starting_line: usize,
}

/// Serializes input into sections.
pub fn get_sections(
    input: &str,
    original_cursor_position: &EditorPosition,
    preferences: &Preferences,
) -> Result<(Vec<MarkdownSection>, usize), Box<dyn Error>> {
    use super::parsing::contents::{append_a_line_break, finish_current_content_section};
    use super::parsing::headings::{
        alternate_headings::get_valid_alternate_heading_level,
        alternate_headings::validation::{
            validate_alternate_sub_heading, validate_alternate_top_heading,
        },
        get_top_heading_level,
        hash_headings::validation::{validate_sub_hash_heading, validate_top_hash_heading},
    };

    if input.is_empty() {
        return Ok((Vec::new(), 0));
    }

    let mut sections: Vec<MarkdownSection> = Vec::new();
    let input_lines: Vec<&str> = input.split('\n').collect();

    let document_top_heading_level = get_top_heading_level(&input_lines);

    // Hash literals
    let mut top_heading_hash_literal = String::new();
    if let Some(document_top_heading_level) = document_top_heading_level {
        top_heading_hash_literal = "#".repeat(document_top_heading_level);
    }

    let mut current_heading_level = 0;

    // Property sections.
    let mut temp_properties = String::new();
    let mut is_reading_property_block = false;

    // Code block sections.
    let mut temp_code_block = String::new();
    let mut is_reading_code_block = false;
    let mut current_code_block_backtick_count: Option<usize> = None;

    // Content sections.
    // Everything goes into `MarkdownSection::Content` type,
    // unless it detects some specific Markdown syntax that needs to be parsed.
    let mut temp_content_section = String::new();
    let mut is_reading_content_section = false;

    let mut error_information = ErrorInformation {
        reading_section_starting_line: 0,
    };

    let mut new_cursor_section_index = 0;

    // Iterate over lines of a document.
    for (index, &line) in input_lines.iter().enumerate() {
        let mut is_cursor_line = index == original_cursor_position.line;
        if (is_cursor_line) {
            new_cursor_section_index = sections.len() - 1;
            console_log!("{:#?}", sections);
            console_log!("{}", new_cursor_section_index);
        }

        // "is_reading_content_section" should be updated in previous iterations.
        if line.is_empty() && !is_reading_content_section && !is_reading_code_block {
            continue;
        }
        is_reading_content_section = true;

        let alternate_heading_level: Option<usize> =
            get_valid_alternate_heading_level(&input_lines, index);

        // Read Properties.
        if sections.is_empty()
            && ((index == 0 && alternate_heading_level.is_none() && line == "---")
                || is_reading_property_block)
        {
            finish_current_content_section(
                &mut is_reading_content_section,
                &mut sections,
                &mut temp_content_section,
            );

            let is_first_property_section_line = temp_properties.is_empty();
            if line == "---" {
                if is_first_property_section_line {
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
                if !is_first_property_section_line {
                    temp_properties.push('\n');
                }
                temp_properties.push_str(line);
                continue;
            }
        }

        // Read code blocks.
        let is_valid_code_block_syntax_line = line.starts_with("```");
        if is_valid_code_block_syntax_line || is_reading_code_block {
            finish_current_content_section(
                &mut is_reading_content_section,
                &mut sections,
                &mut temp_content_section,
            );

            if is_valid_code_block_syntax_line {
                let current_line_backtick_count = line.chars().filter(|&c| c == '`').count();

                let mut closing_pair = false;
                if let Some(reading_code_block_backtick_count) = current_code_block_backtick_count {
                    if is_reading_code_block
                        && current_line_backtick_count == reading_code_block_backtick_count
                    {
                        closing_pair = true;
                    }
                }

                if !is_reading_code_block {
                    // Enter a code block.
                    error_information.reading_section_starting_line = index;
                    temp_code_block.push_str(line);
                    is_reading_code_block = true;
                    current_code_block_backtick_count = Some(current_line_backtick_count);
                    continue;
                } else if closing_pair {
                    // Exit a code block.
                    temp_code_block.push_str(format!("\n{}", line).as_str());
                    sections.push(MarkdownSection::Code(temp_code_block.clone()));
                    current_code_block_backtick_count = None;

                    // Clear temporary code block.
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

        // Read hash headings.
        let is_hash_symbol_only = line.chars().all(|item| item == '#');
        if line.starts_with('#') && (line.contains("# ") || is_hash_symbol_only) {
            if let Some(document_top_heading_level) = document_top_heading_level {
                let is_top_level = validate_top_hash_heading(line, &top_heading_hash_literal);

                if is_top_level {
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
                    let is_sub_heading = validate_sub_hash_heading(line, is_hash_symbol_only);
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

        // Read alternate headings.
        if let Some(alternate_heading_level) = alternate_heading_level {
            if is_reading_code_block || is_reading_property_block {
                continue;
            }

            let mut overriding_temp_content_section: Vec<String> = {
                let temp_content_section_clone = temp_content_section.clone();
                temp_content_section_clone
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect()
            };
            overriding_temp_content_section.pop();
            temp_content_section = overriding_temp_content_section.join("\n");

            finish_current_content_section(
                &mut is_reading_content_section,
                &mut sections,
                &mut temp_content_section,
            );

            if let Some(document_top_heading_level) = document_top_heading_level {
                let is_top_level =
                    validate_alternate_top_heading(&input_lines, index, document_top_heading_level);
                let is_sub_level =
                    validate_alternate_sub_heading(&input_lines, index, document_top_heading_level);

                let previous_line: Option<&str> = if index > 0 {
                    input_lines.get(index - 1).copied()
                } else {
                    None
                };

                if let Some(previous_line) = previous_line {
                    let mut section_string = previous_line.to_string();
                    section_string.push('\n');
                    section_string.push_str(line);

                    if is_top_level {
                        sections.push(MarkdownSection::Heading(HeadingLevel::Top(section_string)));
                        current_heading_level = document_top_heading_level;

                        continue;
                    } else if is_sub_level {
                        if alternate_heading_level > current_heading_level {
                            sections.push(MarkdownSection::Heading(HeadingLevel::FirstSub(
                                section_string,
                            )));
                        } else {
                            sections
                                .push(MarkdownSection::Heading(HeadingLevel::Sub(section_string)));
                        }

                        current_heading_level = alternate_heading_level;
                        continue;
                    }
                }
            }
        }

        // Read contents.
        if is_reading_content_section {
            error_information.reading_section_starting_line = index;

            check_parsing_error(
                is_reading_code_block,
                is_reading_property_block,
                preferences,
                &error_information,
            )?;
            append_a_line_break(&mut temp_content_section, line);
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

    check_parsing_error(
        is_reading_code_block,
        is_reading_property_block,
        preferences,
        &error_information,
    )?;

    Ok((sections, new_cursor_section_index))
}

/// Returns an error if the document is invalid.
fn check_parsing_error(
    is_reading_code_block: bool,
    is_reading_property_block: bool,
    preferences: &Preferences,
    error_information: &ErrorInformation,
) -> Result<(), Box<dyn Error>> {
    use crate::utils::{get_locale_string, LocaleCategory};

    if is_reading_code_block || is_reading_property_block {
        let error_message = if let Some(true) = preferences
            .options
            .other_options
            .show_more_detailed_error_messages
        {
            let mut msg = get_locale_string(
                &preferences.locales,
                LocaleCategory::Parsing,
                "Failed to parse the document. [Line: {LINE_NUMBER}]",
            );
            msg = msg.replace(
                "{LINE_NUMBER}",
                &(error_information.reading_section_starting_line + 1).to_string(),
            );

            return Err(msg.into());
        } else {
            get_locale_string(
                &preferences.locales,
                LocaleCategory::Parsing,
                "Failed to parse the document.",
            )
        };

        return Err(error_message.into());
    }

    Ok(())
}
