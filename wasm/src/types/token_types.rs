#[derive(Debug, PartialEq)]
pub enum HeadingLevel {
    Top(String),
    FirstSub(String),
    Sub(String),
}

#[derive(Debug, PartialEq)]
pub enum MarkdownSection {
    Property(String),
    Heading(HeadingLevel),
    Content(String),
    Code(String),
    Unknown(String),
}
