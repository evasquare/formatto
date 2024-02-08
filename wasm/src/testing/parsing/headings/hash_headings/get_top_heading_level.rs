use crate::{testing::setup, tools::parsing::headings::get_top_heading_level};

#[test]
fn only_hash_heading() {
    setup();

    let input: Vec<&str> = r#"## Heading 2
## Heading 2
## Heading 2"#
        .split('\n')
        .collect();

    let expected_output = 2;

    assert_eq!(get_top_heading_level(&input).unwrap(), expected_output);
}

#[test]
fn hash_headings_and_alternative_headings() {
    setup();

    let input: Vec<&str> = r#"## Heading 2
## Heading 2
## Heading 2

Heading1
====

Heading2
---
"#
    .split('\n')
    .collect();

    let expected_output = 1;

    assert_eq!(get_top_heading_level(&input).unwrap(), expected_output);
}

#[test]
fn invalid_alternative_headings_1() {
    setup();

    let input: Vec<&str> = r#"## Heading 2
## Heading 2
## Heading 2

====
INVALID

---
INVALID
"#
    .split('\n')
    .collect();

    let expected_output = 2;

    assert_eq!(get_top_heading_level(&input).unwrap(), expected_output);
}

#[test]
fn invalid_alternative_headings_2() {
    setup();

    let input: Vec<&str> = r#"## Heading 2
## Heading 2
## Heading 2

INVALID
====
INVALID

INVALID
---
INVALID
"#
    .split('\n')
    .collect();

    let expected_output = 1;

    assert_eq!(get_top_heading_level(&input).unwrap(), expected_output);
}
