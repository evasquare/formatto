use crate::{
    testing::{get_example_settings, setup},
    tools::{
        parsing::get_sections,
        tokens::{HeadingLevel, MarkdownSection},
    },
};

mod get_top_heading_level;

#[test]
fn same_level_hash_headings() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
## Heading 2"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn invalid_hash_headings() {
    setup();

    let input = r#"##Heading 2
###Heading 3
####Heading 4"#;

    let expected_output = vec![MarkdownSection::Content(
        r#"##Heading 2
###Heading 3
####Heading 4"#
            .to_string(),
    )];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn hash_headings_only_1() {
    setup();

    let input = r#"## Heading 2
## Heading 2
## Heading 2"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn hash_headings_only_2() {
    setup();

    let input = r#"## Heading 2
### Heading 3
#### Heading 4"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn sub_hash_headings() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::Sub("### Heading 3".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn random_line_breaks() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3

Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
## Heading 2"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn two_levels_of_hash_headings() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn hash_headings_without_title_names() {
    setup();

    let input = r#"#
##
##
##
##
###
###
####
####
##
#
"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("#".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("##".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("###".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("###".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("####".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("####".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("#".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}
