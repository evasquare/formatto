use serde_json::Value;
use std::error::Error;

use crate::setting_schema::PluginSettings;
use crate::tools::tokens::{HeadingLevel, MarkdownSection};
use crate::{console_error, Preferences};

/// Formats a document based on the parsed sections.
pub fn get_formatted_string(
    sections: Vec<MarkdownSection>,
    preferences: &Preferences,
) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();

    // Check the type of the last parsed section.
    let mut right_after_properties = false;
    let mut right_after_heading = false;
    let mut right_after_code_block = false;

    let settings: &PluginSettings = &preferences.settings;
    let locale: &Value = &preferences.locales;

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
                                parse_string_to_usize(
                                    &settings.other_gaps.after_properties,
                                    locale,
                                )? + 1
                            } else {
                                parse_string_to_usize(
                                    &settings.heading_gaps.before_top_level_headings,
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
                            } else if right_after_properties {
                                parse_string_to_usize(
                                    &settings.other_gaps.after_properties,
                                    locale,
                                )? + 1
                            } else {
                                parse_string_to_usize(
                                    &settings.heading_gaps.before_first_sub_heading,
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
                            } else if right_after_properties {
                                parse_string_to_usize(
                                    &settings.other_gaps.after_properties,
                                    locale,
                                )? + 1
                            } else {
                                parse_string_to_usize(
                                    &settings.heading_gaps.before_sub_headings,
                                    locale,
                                )? + 1
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
                        parse_string_to_usize(&settings.other_gaps.after_properties, locale)? + 1
                    } else if right_after_code_block {
                        parse_string_to_usize(
                            &settings.other_gaps.before_contents_after_code_blocks,
                            locale,
                        )? + 1
                    } else {
                        parse_string_to_usize(&settings.other_gaps.before_contents, locale)? + 1
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
                        parse_string_to_usize(&settings.other_gaps.after_properties, locale)? + 1
                    } else if right_after_heading {
                        parse_string_to_usize(
                            &settings.other_gaps.before_code_blocks_after_headings,
                            locale,
                        )? + 1
                    } else {
                        parse_string_to_usize(&settings.other_gaps.before_code_blocks, locale)? + 1
                    },
                    0,
                ));

                right_after_properties = false;
                right_after_heading = false;
                right_after_code_block = true
            }
        }
    }

    if preferences.settings.format_options.insert_newline == Some(true) {
        output.push('\n');
    }

    Ok(output)
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

    if let Some(input) = input {
        if input.is_empty() {
            let msg = get_locale_string(
                locales,
                LocaleCategory::Formatting,
                "Failed to read options. Please make sure there is no option with an empty value.",
            );

            return Err(msg.into());
        }
    }

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

                #[allow(clippy::needless_return)]
                return Err(msg.into());
            }
        },
        None => {
            let msg = get_locale_string(
                locales,
                LocaleCategory::Formatting,
                "Failed to read option properties.",
            );

            #[allow(clippy::needless_return)]
            return Err(msg.into());
        }
    }
}
