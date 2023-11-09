use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyGaps {
    pub after_properties: Option<String>,
}

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
    pub contents_after_headings: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainPluginSettings {
    pub property_gaps: PropertyGaps,
    pub heading_gaps: HeadingGaps,
    pub other_gaps: OtherGaps,
}
