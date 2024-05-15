use crate::{
    testing::{get_example_preferences, setup},
    tools::parsing::get_sections,
};

#[test]
fn empty_input() {
    setup();

    let input = "";
    let expected_output = Vec::new();

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}
