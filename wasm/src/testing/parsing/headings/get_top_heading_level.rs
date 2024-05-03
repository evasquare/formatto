use crate::{testing::setup, tools::parsing::headings::get_top_heading_level};

/// Hash headings only.
#[test]
fn case_1() {
    setup();

    let input: Vec<&str> = r#"## Heading 2
## Heading 2
## Heading 2"#
        .split('\n')
        .collect();

    let expected_output = 2;

    assert_eq!(get_top_heading_level(&input).unwrap(), expected_output);
}

/// Hash headings and alternate headings are mixed.
#[test]
fn case_2() {
    setup();

    let input: Vec<&str> = r#"## Heading 2
## Heading 2
## Heading 2

Heading 1
===

Heading 2
---
"#
    .split('\n')
    .collect();

    let expected_output = 1;

    assert_eq!(get_top_heading_level(&input).unwrap(), expected_output);
}

#[test]
fn case_3() {
    setup();

    let input: Vec<&str> = r#"````rust
```compile_fail
# struct MyNonSendType(std::rc::Rc<()>);
fn is_send<T: Send>() {}
is_send::<MyNonSendType>();
```
````

## Heading 2
"#
    .split('\n')
    .collect();

    let expected_output = 2;

    assert_eq!(get_top_heading_level(&input).unwrap(), expected_output);
}

#[test]
fn case_4() {
    setup();

    let input: Vec<&str> = r#"````md
```language-name
// code
```
````

## Heading 2
"#
    .split('\n')
    .collect();

    let expected_output = 2;

    assert_eq!(get_top_heading_level(&input).unwrap(), expected_output);
}

#[test]
fn invalid_input_1() {
    setup();

    let input: Vec<&str> = r#"## Heading 2
## Heading 2
## Heading 2

===
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
fn invalid_input_2() {
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
