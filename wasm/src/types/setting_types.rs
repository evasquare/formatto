use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadingGaps {
    pub top_level_headings: Option<String>,
    pub first_sub_heading: Option<String>,
    pub sub_headings: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherGaps {
    pub after_properties: Option<String>,
    pub contents_after_headings: Option<String>,
    pub before_code_blocks: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainPluginSettings {
    pub heading_gaps: HeadingGaps,
    pub other_gaps: OtherGaps,
}
