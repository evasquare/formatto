use std::error::Error;

use crate::console_error;
use crate::setting_schema::MainPluginSettings;
use crate::tools::tokens::{HeadingLevel, MarkdownSection};

/// Return a String value that is replacing the entire document.
pub fn get_formatted_string(
    sections: Vec<MarkdownSection>,
    settings: &MainPluginSettings,
) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();

    let mut right_after_properties = false;
    let mut right_after_heading = false;
    let mut right_after_code_block = false;

    for section in sections {
        match section {
            MarkdownSection::Property(content) => {
                output.push_str(&content);

                right_after_properties = true;
                right_after_heading = false;
                right_after_code_block = false;
            }
            MarkdownSection::Heading(heading_level) => {
                match heading_level {
                    HeadingLevel::Top(content) => {
                        output.push_str(&insert_line_breaks(
                            &content,
                            if output.is_empty() {
                                0
                            } else if right_after_properties {
                                parse_string_to_usize(&settings.other_gaps.after_properties)? + 1
                            } else {
                                parse_string_to_usize(
                                    &settings.heading_gaps.before_top_level_headings,
                                )? + 1
                            },
                            0,
                        ));
                    }
                    HeadingLevel::FirstSub(content) => {
                        let formatted = insert_line_breaks(
                            &content,
                            if output.is_empty() {
                                0
                            } else if right_after_properties {
                                parse_string_to_usize(&settings.other_gaps.after_properties)? + 1
                            } else {
                                parse_string_to_usize(
                                    &settings.heading_gaps.before_first_sub_heading,
                                )? + 1
                            },
                            0,
                        );
                        output.push_str(&formatted);
                    }
                    HeadingLevel::Sub(content) => {
                        output.push_str(&insert_line_breaks(
                            &content,
                            if output.is_empty() {
                                0
                            } else if right_after_properties {
                                parse_string_to_usize(&settings.other_gaps.after_properties)? + 1
                            } else {
                                parse_string_to_usize(&settings.heading_gaps.before_sub_headings)?
                                    + 1
                            },
                            0,
                        ));
                    }
                }

                right_after_properties = false;
                right_after_heading = true;
                right_after_code_block = false;
            }
            MarkdownSection::Content(content) => {
                output.push_str(&insert_line_breaks(
                    &content,
                    if output.is_empty() {
                        0
                    } else if right_after_properties {
                        parse_string_to_usize(&settings.other_gaps.after_properties)? + 1
                    } else if right_after_code_block {
                        parse_string_to_usize(
                            &settings.other_gaps.before_contents_after_code_blocks,
                        )? + 1
                    } else {
                        parse_string_to_usize(&settings.other_gaps.before_contents)? + 1
                    },
                    0,
                ));

                right_after_properties = false;
                right_after_heading = false;
                right_after_code_block = false;
            }
            MarkdownSection::Code(content) => {
                output.push_str(&insert_line_breaks(
                    &content,
                    if output.is_empty() {
                        0
                    } else if right_after_properties {
                        parse_string_to_usize(&settings.other_gaps.after_properties)? + 1
                    } else if right_after_heading {
                        parse_string_to_usize(
                            &settings.other_gaps.before_code_blocks_after_headings,
                        )? + 1
                    } else {
                        parse_string_to_usize(&settings.other_gaps.before_code_blocks)? + 1
                    },
                    0,
                ));

                right_after_properties = false;
                right_after_heading = false;
                right_after_code_block = true
            }
        }
    }

    if settings.format_options.insert_newline == Some(true) {
        output.push('\n');
    }

    Ok(output)
}

/// Insert line breaks before and after an input.
pub fn insert_line_breaks(input: &str, before: usize, after: usize) -> String {
    let line_breaks_before = "\n".repeat(before);
    let line_breaks_after = "\n".repeat(after);

    format!("{}{}{}", line_breaks_before, input, line_breaks_after)
}

/// Parse a usize value from a &str type argument.
/// Also return an `Error` to handle exceptions.
pub fn parse_string_to_usize(input: &Option<String>) -> Result<usize, Box<dyn Error>> {
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
