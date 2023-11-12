use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadingGaps {
    pub before_top_level_headings: Option<String>,
    pub before_first_sub_heading: Option<String>,
    pub before_sub_headings: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherGaps {
    pub after_properties: Option<String>,
    pub before_contents: Option<String>,
    pub before_contents_after_code_blocks: Option<String>,
    pub before_code_blocks: Option<String>,
    pub before_code_blocks_after_headings: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainPluginSettings {
    pub heading_gaps: HeadingGaps,
    pub other_gaps: OtherGaps,
}
