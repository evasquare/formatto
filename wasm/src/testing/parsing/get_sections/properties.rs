use crate::{
    testing::{get_example_settings, setup},
    tools::{
        parsing::get_sections,
        tokens::{HeadingLevel, MarkdownSection},
    },
};

/// Property section only.
#[test]
fn case_1() {
    setup();

    let input = r#"---
aliases:
- Test
---
"#;

    let expected_output = vec![MarkdownSection::Property(
        "---\naliases:\n- Test\n---".to_string(),
    )];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

/// Properties with other sections.
#[test]
fn case_2() {
    setup();

    let input = r#"---
aliases:
- Test
---

## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3

Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
## Heading 2"#;

    let expected_output = vec![
        MarkdownSection::Property("---\naliases:\n- Test\n---".to_string()),
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

/// Invalid property syntax.
#[test]
fn invalid_input_1() {
    setup();

    let input = r#"---INVALID
aliases:
---
- Test
---INVALID
---INVALID
---INVALID

## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3

Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
## Heading 2"#;

    println!("{}", get_sections(input, &get_example_settings()).is_err());
    assert!(get_sections(input, &get_example_settings()).is_err())
}
