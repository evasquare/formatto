use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadingGaps {
    /// Decides gaps before highest level of headings.
    pub before_top_level_headings: Option<String>,
    /// Decides the child heading gap right before a parent heading.
    pub before_first_sub_heading: Option<String>,
    /// Decides gaps before headings that are not in the highest level.
    pub before_sub_headings: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherGaps {
    /// Decides the gap after a YAML properties.
    pub after_properties: Option<String>,
    /// Decides gaps before contents (ex: Text section right before headings).
    pub before_contents: Option<String>,
    /// Decides gaps before contents that are right after code blocks.
    pub before_contents_after_code_blocks: Option<String>,
    /// Decides gaps before code blocks.
    pub before_code_blocks: Option<String>,
    /// Decides gaps before code blocks that are right after headings.
    pub before_code_blocks_after_headings: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalSettings {
    pub add_empty_line: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainPluginSettings {
    pub heading_gaps: HeadingGaps,
    pub other_gaps: OtherGaps,
    pub additional_settings: AdditionalSettings,
}
