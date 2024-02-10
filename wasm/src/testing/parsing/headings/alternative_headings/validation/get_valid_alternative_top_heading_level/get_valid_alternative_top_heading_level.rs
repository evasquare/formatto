use crate::tools::parsing::headings::alternative_headings::validation::get_valid_alternative_top_heading_level::get_valid_alternative_top_heading_level;

#[test]
fn case_1() {
    let input: Vec<&str> = r#"## Heading 2
## Heading 2
## Heading 2

Heading 1
==="#
        .split('\n')
        .collect();

    let output = get_valid_alternative_top_heading_level(&input, 5).unwrap();
    let expected_out: usize = 1;

    assert_eq!(output, expected_out);
}

#[test]
fn case_2() {
    let input: Vec<&str> = r#"## Heading 2
## Heading 2
## Heading 2

Heading 2
---"#
        .split('\n')
        .collect();

    let output = get_valid_alternative_top_heading_level(&input, 5).unwrap();
    let expected_out: usize = 2;

    assert_eq!(output, expected_out);
}

#[test]
fn case_3() {
    let input: Vec<&str> = r#"## Heading 2
## Heading 2
## Heading 2

Heading 1
===
Heading 2
---"#
        .split('\n')
        .collect();

    let output = get_valid_alternative_top_heading_level(&input, 5).unwrap();
    let expected_out: usize = 1;

    assert_eq!(output, expected_out);
}

#[test]
fn case_4() {
    let input: Vec<&str> = r#"## Heading 2
# Heading 1
### Heading 3

Heading 2
---"#
        .split('\n')
        .collect();

    let output = get_valid_alternative_top_heading_level(&input, 5).unwrap();
    let expected_out: usize = 2;

    assert_eq!(output, expected_out);
}
