//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use std::{assert, vec};

use formatto_wasm::divide_top_headings;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

mod divide_top_headings {
    #[wasm_bindgen_test]
    fn random_line_breaks() {
        let input = r#"## Heading2
Hi everyone

### Heading 3

end of heading 3
#### Heading 4
## Heading 2"#;

        let expected_output = vec![
            vec![
                "## Heading2",
                "Hi everyone",
                "### Heading 3",
                "end of heading 3",
                "#### Heading 4",
            ],
            vec!["## Heading 2"],
        ];

        assert_eq!(divide_top_headings(input), expected_output);
    }

    #[wasm_bindgen_test]
    fn no_subheadings() {
        let input = r#"## Heading2
Hi everyone
## Heading 2"#;

        let expected_output = vec![vec!["## Heading2", "Hi everyone"], vec!["## Heading 2"]];

        assert_eq!(divide_top_headings(input), expected_output);
    }

    #[wasm_bindgen_test]
    fn single_section() {
        let input = r#"## Heading2
Hi everyone
### Subheading
Text under subheading"#;

        let expected_output = vec![vec![
            "## Heading2",
            "Hi everyone",
            "### Subheading",
            "Text under subheading",
        ]];

        assert_eq!(divide_top_headings(input), expected_output);
    }

    #[wasm_bindgen_test]
    fn empty_input() {
        let input = "";

        let expected_output: Vec<Vec<&str>> = vec![];

        assert_eq!(divide_top_headings(input), expected_output);
    }
}
