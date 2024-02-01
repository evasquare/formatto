use crate::{
    setting_schema::{FormatOptions, HeadingGaps, MainPluginSettings, OtherGaps, OtherOptions},
    utils::set_panic_hook,
};

mod formatting;
mod parsing;

#[allow(dead_code)]
fn setup() {
    set_panic_hook();
}

#[allow(dead_code)]
fn get_example_settings() -> MainPluginSettings {
    MainPluginSettings {
        heading_gaps: HeadingGaps {
            before_top_level_headings: Some("3".to_string()),
            before_first_sub_heading: Some("1".to_string()),
            before_sub_headings: Some("2".to_string()),
        },
        other_gaps: OtherGaps {
            after_properties: Some("2".to_string()),
            before_contents: Some("0".to_string()),
            before_contents_after_code_blocks: Some("1".to_string()),
            before_code_blocks: Some("1".to_string()),
            before_code_blocks_after_headings: Some("0".to_string()),
        },
        format_options: FormatOptions {
            insert_newline: Some(false),
        },
        other_options: OtherOptions {
            notify_when_unchanged: Some(false),
            show_more_detailed_error_messages: Some(false),
        },
    }
}
