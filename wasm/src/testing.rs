use crate::utils::{set_panic_hook, Preferences};
use serde_json::Value;

use crate::option_schema::{FormatOptions, HeadingGaps, OtherGaps, OtherOptions, PluginOptions};

mod formatting;
mod parsing;
mod utils;

#[allow(dead_code)]
fn setup() {
    set_panic_hook();
}

/// Returns an example value for testing.
fn get_example_preferences() -> Preferences {
    Preferences {
        options: get_example_options(),
        locales: get_example_locale(),
    }
}

/// Returns an example option value.
fn get_example_options() -> PluginOptions {
    PluginOptions {
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

/// Returns an example locale value.
fn get_example_locale() -> Value {
    let val = r#"
    {
        "parsing": {
            "Failed to parse the document. [Line: {LINE_NUMBER}]": "문서를 읽지 못했습니다. [줄: {LINE_NUMBER}]",
            "Failed to parse the document.": "문서를 읽지 못했습니다."
        },
        "formatting": {
            "Failed to read options. Please make sure there is no option with an empty value.": "옵션을 읽지 못했습니다. 값이 비어있는 옵션이 없는지 다시 확인해주세요.",
            "Failed to read options. Some of them are possibly not positive number values.": "설정을 읽지 못했습니다. 양수가 아닌 값이 있을수도 있습니다.",
            "Failed to read option properties.": "옵션 프로퍼티를 읽지 못했습니다."
        }
    }
    "#;

    serde_json::from_str(val).unwrap()
}
