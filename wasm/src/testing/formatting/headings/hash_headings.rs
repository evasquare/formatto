use crate::{
    testing::{get_example_preferences, setup},
    tools::{formatting::get_formatted_string, parsing::get_sections},
};

/// Multiple hash headings.
#[test]
fn case_1() {
    setup();

    let input = r#"## Heading 2
### Heading 3
#### Heading 4"#;
    let sections = get_sections(input, &get_example_preferences()).unwrap();
    let output = get_formatted_string(sections, &get_example_preferences()).unwrap();
    let expected_output = r#"## Heading 2

### Heading 3

#### Heading 4"#;

    assert_eq!(output, expected_output);
}

#[test]
fn invalid_input_1() {
    setup();

    let input = r#"##Heading 2
###Heading 3
####Heading 4"#;
    let sections = get_sections(input, &get_example_preferences()).unwrap();
    let output = get_formatted_string(sections, &get_example_preferences()).unwrap();
    let expected_output = r#"##Heading 2
###Heading 3
####Heading 4"#;

    assert_eq!(output, expected_output);
}
