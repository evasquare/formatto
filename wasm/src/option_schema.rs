use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadingGaps {
    /// "Decides the gap before a top-level heading."
    pub before_top_level_headings: Option<String>,
    /// Decides the child heading gap right before a parent heading.
    pub before_first_sub_heading: Option<String>,
    /// Decides gaps before headings that are not top-level.
    pub before_sub_headings: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherGaps {
    /// Decides the gap after a property section.
    pub after_properties: Option<String>,
    /// Decides gaps before content sections. (ex: Text before headings)
    pub before_contents: Option<String>,
    /// Decides gaps before \"contents that are after code blocks.\"
    pub before_contents_after_code_blocks: Option<String>,
    /// Decides gaps before code blocks.
    pub before_code_blocks: Option<String>,
    /// Decides gaps before \"code blocks that are after headings.\"
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
    #[allow(dead_code)]
    /// Displays a different message when no change is needed.
    pub notify_when_unchanged: Option<bool>,
    /// Displays additional information when parsing fails.
    pub show_more_detailed_error_messages: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginOptions {
    pub heading_gaps: HeadingGaps,
    pub other_gaps: OtherGaps,
    pub format_options: FormatOptions,
    pub other_options: OtherOptions,
}
