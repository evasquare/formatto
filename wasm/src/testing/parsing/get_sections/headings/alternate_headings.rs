use crate::{
    testing::{get_example_settings, setup},
    tools::{
        parsing::get_sections,
        tokens::{HeadingLevel, MarkdownSection},
    },
};

#[test]
fn case_1() {
    setup();

    let input = r#"Heading 1
====

### Heading 3
#### Heading 4"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("Heading 1\n====".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn case_2() {
    setup();

    let input = r#"Heading 1
====

Heading 2
-------

### Heading 3
#### Heading 4
# Heading 1
## Heading 2
"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("Heading 1\n====".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("Heading 2\n-------".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("# Heading 1".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("## Heading 2".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn case_3() {
    setup();

    let input = r#"## Heading 2

### Heading 3
```ts
console.log("Hello World");
```

aaabbbccc

Content
===
Content
---


## Heading 2


## Heading 2


## Heading 2"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::FirstSub("## Heading 2".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Code("```ts\nconsole.log(\"Hello World\");\n```".to_string()),
        MarkdownSection::Content("aaabbbccc".to_string()),
        MarkdownSection::Heading(HeadingLevel::Top("Content\n===".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("Content\n---".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("## Heading 2".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("## Heading 2".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("## Heading 2".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn edge_case_1() {
    setup();

    let input = r#"## Heading 2
aabbcc
===
Content
---

# Heading 1
"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::FirstSub("## Heading 2".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("aabbcc\n===".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("Content\n---".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("# Heading 1".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn edge_case_2() {
    setup();

    let input = r#"## Heading 2
INVALID
INVALID
===
Content
---
"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content("INVALID\nINVALID\n===\nContent\n---".to_string()),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}

#[test]
fn edge_case_3() {
    setup();

    let input = r#"## Heading 2

### Heading 3
```ts
console.log("Hello World");
```

aaabbbccc
Content
===
Content
---
--–
Content
===



## Heading 2



## Heading 2



## Heading 2
"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top(String::from("## Heading 2"))),
        MarkdownSection::Heading(HeadingLevel::FirstSub(String::from("### Heading 3"))),
        MarkdownSection::Code(String::from("```ts\nconsole.log(\"Hello World\");\n```")),
        MarkdownSection::Content(String::from(
            "aaabbbccc\nContent\n===\nContent\n---\n--–\nContent\n===",
        )),
        MarkdownSection::Heading(HeadingLevel::Top(String::from("## Heading 2"))),
        MarkdownSection::Heading(HeadingLevel::Top(String::from("## Heading 2"))),
        MarkdownSection::Heading(HeadingLevel::Top(String::from("## Heading 2"))),
    ];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}
