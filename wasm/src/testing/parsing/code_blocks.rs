use crate::{
    testing::{get_example_settings, setup},
    tools::{
        parsing::get_sections,
        tokens::{HeadingLevel, MarkdownSection},
    },
};

mod invalid_input;

#[test]
fn code_blocks_1() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
```rust
fn main(
println!(\"Hello World\");
) {}
```"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
        MarkdownSection::Code(
            r#"```rust
fn main(
println!(\"Hello World\");
) {}
```"#
                .to_string(),
        ),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}
