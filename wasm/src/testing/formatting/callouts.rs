use crate::{
    testing::{get_example_preferences, setup},
    tools::{formatting::get_formatted_string, parsing::get_sections},
};

#[test]
fn case_1() {
    setup();

    let input = r#"## Heading 2

Lorem Ipsum is simply dummy text of the printing and typesetting industry.
> Callout 1

> Callout 2```"#;

    let sections = get_sections(input, &get_example_preferences()).unwrap();
    let output = get_formatted_string(sections, &get_example_preferences()).unwrap();
    let expected_output = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

> Callout 1

> Callout 2```"#;

    assert_eq!(output, expected_output);
}

#[test]
fn case_2() {
    setup();

    let input = r#"> Callout 1




> Callout 2
> Callout 2




> Callout 3
> Callout 3
> Callout 3
"#;

    let sections = get_sections(input, &get_example_preferences()).unwrap();
    let output = get_formatted_string(sections, &get_example_preferences()).unwrap();
    let expected_output = r#"> Callout 1

> Callout 2
> Callout 2

> Callout 3
> Callout 3
> Callout 3"#;

    assert_eq!(output, expected_output);
}
