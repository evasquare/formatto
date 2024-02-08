use crate::{
    testing::{get_example_settings, setup},
    tools::parsing::get_sections,
};

#[test]
fn empty_input() {
    setup();

    let input = "";
    let expected_output = vec![];

    assert_eq!(
        get_sections(input, &get_example_settings()).unwrap(),
        expected_output
    );
}
