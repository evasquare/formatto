use crate::{
    testing::{get_example_preferences, setup},
    tools::{
        parsing::get_sections,
        tokens::{HeadingLevel, MarkdownSection},
    },
};

#[test]
fn case_1() {
    setup();

    let input = r#"## Heading 2

Lorem Ipsum is simply dummy text of the printing and typesetting industry.

> Callout 1

> Callout 2"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Callout("> Callout 1".to_string()),
        MarkdownSection::Callout("> Callout 2".to_string()),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_2() {
    setup();

    let input = r#"
> Callout 1

> Callout 2
> Callout 2

> Callout 3
> Callout 3
> Callout 3"#;

    let expected_output = vec![
        MarkdownSection::Callout("> Callout 1".to_string()),
        MarkdownSection::Callout("> Callout 2\n> Callout 2".to_string()),
        MarkdownSection::Callout("> Callout 3\n> Callout 3\n> Callout 3".to_string()),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_3() {
    setup();

    let input = r#"
> Callout 1

> Callout 2
> Callout 2
```ts
console.log("Hello, World!");
```"#;

    let expected_output = vec![
        MarkdownSection::Callout("> Callout 1".to_string()),
        MarkdownSection::Callout("> Callout 2\n> Callout 2".to_string()),
        MarkdownSection::Code("```ts\nconsole.log(\"Hello, World!\");\n```".to_string()),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_4() {
    setup();

    let input = r#"> Callout 1
# Heading 1




> Callout 2
> Callout 2




> Callout 3
> Callout 3
> Callout 3
"#;

    let expected_output = vec![
        MarkdownSection::Callout("> Callout 1".to_string()),
        MarkdownSection::Heading(HeadingLevel::Top("# Heading 1".to_string())),
        MarkdownSection::Callout("> Callout 2\n> Callout 2".to_string()),
        MarkdownSection::Callout("> Callout 3\n> Callout 3\n> Callout 3".to_string()),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_5() {
    setup();

    let input = r#"> Callout 1
# Heading 1




> Callout 2
> Callout 2
```ts
console.log("Hello, World!");
```



> Callout 3
> Callout 3
> Callout 3
"#;

    let expected_output = vec![
        MarkdownSection::Callout("> Callout 1".to_string()),
        MarkdownSection::Heading(HeadingLevel::Top("# Heading 1".to_string())),
        MarkdownSection::Callout("> Callout 2\n> Callout 2".to_string()),
        MarkdownSection::Code("```ts\nconsole.log(\"Hello, World!\");\n```".to_string()),
        MarkdownSection::Callout("> Callout 3\n> Callout 3\n> Callout 3".to_string()),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_6() {
    setup();

    let input = r#"> Callout 1

# Heading 1




> Callout 2
> Callout 2
```ts
console.log("Hello, World!");
```



> Callout 3
> Callout 3
> Callout 3
"#;

    let expected_output = vec![
        MarkdownSection::Callout("> Callout 1".to_string()),
        MarkdownSection::Heading(HeadingLevel::Top("# Heading 1".to_string())),
        MarkdownSection::Callout("> Callout 2\n> Callout 2".to_string()),
        MarkdownSection::Code("```ts\nconsole.log(\"Hello, World!\");\n```".to_string()),
        MarkdownSection::Callout("> Callout 3\n> Callout 3\n> Callout 3".to_string()),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_7() {
    setup();

    let input = r#"> Callout 1
> Callout 1
> Callout 1
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

    let expected_output = vec![
        MarkdownSection::Callout("> Callout 1\n> Callout 1\n> Callout 1".to_string()),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_8() {
    setup();

    let input = r#"> Callout 1
> Callout 1
> Callout 1
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
> Callout 1
> Callout 1
> Callout 1
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

    let expected_output = vec![
        MarkdownSection::Callout("> Callout 1\n> Callout 1\n> Callout 1".to_string()),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Callout("> Callout 1\n> Callout 1\n> Callout 1".to_string()),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}
