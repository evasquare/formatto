use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadingGaps {
    /// Decides gaps before top level of headings.
    pub before_top_level_headings: Option<String>,
    /// Decides the child heading gap right before a parent heading.
    pub before_first_sub_heading: Option<String>,
    /// Decides gaps before headings that are not in the top level.
    pub before_sub_headings: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherGaps {
    /// Decides the gap after the property section.
    pub after_properties: Option<String>,
    /// Decides gaps before contents (ex: Text section before headings).
    pub before_contents: Option<String>,
    /// Decides gaps before 'contents that are after code blocks'.
    pub before_contents_after_code_blocks: Option<String>,
    /// Decides gaps before code blocks.
    pub before_code_blocks: Option<String>,
    /// Decides gaps before 'code blocks that are after headings'.
    pub before_code_blocks_after_headings: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatOptions {
    /// Inserts a newline at the end of a document.
    pub insert_newline: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherOptions {
    /// Displays a different message when no change was made.
    pub notify_when_unchanged: Option<bool>,
    /// Displays additional information when parsing fails.
    pub show_more_detailed_error_messages: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainPluginSettings {
    pub heading_gaps: HeadingGaps,
    pub other_gaps: OtherGaps,
    pub format_options: FormatOptions,
    pub other_options: OtherOptions,
}
