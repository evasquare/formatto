use crate::{
    testing::{get_example_locale, get_example_settings, setup},
    tools::{formatting::get_formatted_string, parsing::get_sections},
};

#[test]
fn case_1() {
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

    let sections = get_sections(input, &get_example_settings(), &get_example_locale()).unwrap();
    let output =
        get_formatted_string(sections, &get_example_settings(), &get_example_locale()).unwrap();
    let expected_output = r#"---
aliases:
    - Test
---


## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4



## Heading 2"#;

    assert_eq!(output, expected_output);
}

#[test]
fn case_2() {
    setup();

    let input = r#"---
aliases:
  - Test
---"#;

    let sections = get_sections(input, &get_example_settings(), &get_example_locale()).unwrap();
    let output =
        get_formatted_string(sections, &get_example_settings(), &get_example_locale()).unwrap();
    let expected_output = r#"---
aliases:
  - Test
---"#;

    assert_eq!(output, expected_output);
}
