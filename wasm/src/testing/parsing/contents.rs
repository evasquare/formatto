use crate::{
    testing::{get_example_settings, setup},
    tools::{
        parsing::get_sections,
        tokens::{HeadingLevel, MarkdownSection},
    },
};

#[test]
fn contents_only() {
    setup();

    let input = r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

    let expected_output = vec![MarkdownSection::Content(
        r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#
            .to_string(),
    )];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn contents_with_line_break_syntax() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

\
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
        MarkdownSection::Content(
            r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry.

\
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#
                .to_string(),
        ),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}
