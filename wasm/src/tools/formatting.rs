use serde_json::Value;
use std::error::Error;

use crate::tools::tokens::{HeadingLevel, MarkdownSection};
use crate::{console_error, Preferences};

/// Formats a parsed document.
pub fn get_formatted_string(
    sections: Vec<MarkdownSection>,
    new_cursor_section_index: usize,
    preferences: &Preferences,
) -> Result<(String, usize), Box<dyn Error>> {
    let mut output = String::new();

    // Check which type of section was last parsed.
    let mut is_right_after_properties = false;
    let mut is_right_after_heading = false;
    let mut is_right_after_code_block = false;

    let options = &preferences.options;
    let locale = &preferences.locales;

    let mut new_line_index = 0;

    for (index, section) in sections.iter().enumerate() {
        match section {
            MarkdownSection::Property(content) => {
                output.push_str(&content);

                is_right_after_properties = true;
                is_right_after_heading = false;
                is_right_after_code_block = false;
            }
            MarkdownSection::Heading(heading_level) => {
                match heading_level {
                    HeadingLevel::Top(content) => {
                        output.push_str(&insert_line_breaks(
                            &content,
                            if output.is_empty() {
                                0
                            } else if is_right_after_properties {
                                parse_string_to_usize(&options.other_gaps.after_properties, locale)?
                                    + 1
                            } else {
                                parse_string_to_usize(
                                    &options.heading_gaps.before_top_level_headings,
                                    locale,
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
                            } else if is_right_after_properties {
                                parse_string_to_usize(&options.other_gaps.after_properties, locale)?
                                    + 1
                            } else {
                                parse_string_to_usize(
                                    &options.heading_gaps.before_first_sub_heading,
                                    locale,
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
                            } else if is_right_after_properties {
                                parse_string_to_usize(&options.other_gaps.after_properties, locale)?
                                    + 1
                            } else {
                                parse_string_to_usize(
                                    &options.heading_gaps.before_sub_headings,
                                    locale,
                                )? + 1
                            },
                            0,
                        ));
                    }
                }

                is_right_after_properties = false;
                is_right_after_heading = true;
                is_right_after_code_block = false;
            }
            MarkdownSection::Content(content) => {
                output.push_str(&insert_line_breaks(
                    &content,
                    if output.is_empty() {
                        0
                    } else if is_right_after_properties {
                        parse_string_to_usize(&options.other_gaps.after_properties, locale)? + 1
                    } else if is_right_after_code_block {
                        parse_string_to_usize(
                            &options.other_gaps.before_contents_after_code_blocks,
                            locale,
                        )? + 1
                    } else {
                        parse_string_to_usize(&options.other_gaps.before_contents, locale)? + 1
                    },
                    0,
                ));

                is_right_after_properties = false;
                is_right_after_heading = false;
                is_right_after_code_block = false;
            }
            MarkdownSection::Code(content) => {
                output.push_str(&insert_line_breaks(
                    &content,
                    if output.is_empty() {
                        0
                    } else if is_right_after_properties {
                        parse_string_to_usize(&options.other_gaps.after_properties, locale)? + 1
                    } else if is_right_after_heading {
                        parse_string_to_usize(
                            &options.other_gaps.before_code_blocks_after_headings,
                            locale,
                        )? + 1
                    } else {
                        parse_string_to_usize(&options.other_gaps.before_code_blocks, locale)? + 1
                    },
                    0,
                ));

                is_right_after_properties = false;
                is_right_after_heading = false;
                is_right_after_code_block = true
            }
        }

        if index == new_cursor_section_index {
            let a: Vec<&str> = output.split("\n").collect();
            new_line_index = a.len() - 1;
            console_error!("{:#?}", a);
            console_error!("{}", new_line_index)
        }
    }

    if preferences.options.format_options.insert_newline == Some(true) {
        output.push('\n');
    }

    Ok((output, new_line_index))
}

/// Inserts line breaks before and after an input.
pub fn insert_line_breaks(input: &str, before_count: usize, after_count: usize) -> String {
    let line_breaks_before = "\n".repeat(before_count);
    let line_breaks_after = "\n".repeat(after_count);

    format!("{}{}{}", line_breaks_before, input, line_breaks_after)
}

/// Parses a usize value from a &str type argument.
pub fn parse_string_to_usize(
    input: &Option<String>,
    locales: &Value,
) -> Result<usize, Box<dyn Error>> {
    use crate::utils::{get_locale_string, LocaleCategory};

    match input {
        Some(input) => match input.parse::<usize>() {
            Ok(num) => Ok(num),
            Err(err) => {
                console_error!("{}", err);

                let msg = get_locale_string(
                    locales,
                    LocaleCategory::Formatting,
                    "Failed to read options. Some of them are possibly not positive number values.",
                );

                Err(msg.into())
            }
        },
        None => unreachable!(),
    }
}
