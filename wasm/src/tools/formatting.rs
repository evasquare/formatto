use std::error::Error;

use crate::setting_schema::MainPluginSettings;
use crate::tools::{
    parsing::{insert_line_breaks, parse_str_to_usize},
    tokens::{HeadingLevel, MarkdownSection},
};

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
                                parse_str_to_usize(&settings.other_gaps.after_properties)? + 1
                            } else {
                                parse_str_to_usize(
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
                                parse_str_to_usize(&settings.other_gaps.after_properties)? + 1
                            } else {
                                parse_str_to_usize(&settings.heading_gaps.before_first_sub_heading)?
                                    + 1
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
                                parse_str_to_usize(&settings.other_gaps.after_properties)? + 1
                            } else {
                                parse_str_to_usize(&settings.heading_gaps.before_sub_headings)? + 1
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
                        parse_str_to_usize(&settings.other_gaps.after_properties)? + 1
                    } else if right_after_code_block {
                        parse_str_to_usize(&settings.other_gaps.before_contents_after_code_blocks)?
                            + 1
                    } else {
                        parse_str_to_usize(&settings.other_gaps.before_contents)? + 1
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
                        parse_str_to_usize(&settings.other_gaps.after_properties)? + 1
                    } else if right_after_heading {
                        parse_str_to_usize(&settings.other_gaps.before_code_blocks_after_headings)?
                            + 1
                    } else {
                        parse_str_to_usize(&settings.other_gaps.before_code_blocks)? + 1
                    },
                    0,
                ));

                right_after_properties = false;
                right_after_heading = false;
                right_after_code_block = true
            }
        }
    }

    Ok(output)
}
