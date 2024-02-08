use crate::{
    testing::{get_example_settings, setup},
    tools::parsing::get_sections,
};

#[test]
fn invalid_input_1() {
    setup();

    let input = r#"```
code
SPACE```"#;
    let sections = get_sections(input, &get_example_settings());
    assert!(sections.is_err());
}
