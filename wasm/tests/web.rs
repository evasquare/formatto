//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

mod divide_top_headings {
    use formatto_wasm::format_tools::divide_top_headings;
    use formatto_wasm::{HeadingLevel, MarkdownSection};

    use std::{assert, vec};
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn random_line_breaks() {
        let input = r#"## Heading 2
Hi everyone

### Heading 3

end of heading 3
#### Heading 4
## Heading 2"#;

        let expected_output = vec![
            vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2")),
                MarkdownSection::Unknown("Hi everyone"),
                MarkdownSection::Unknown("### Heading 3"),
                MarkdownSection::Unknown("end of heading 3"),
                MarkdownSection::Unknown("#### Heading 4"),
            ],
            vec![MarkdownSection::Heading(HeadingLevel::Top("## Heading 2"))],
        ];

        assert_eq!(divide_top_headings(input), expected_output);
    }

    #[wasm_bindgen_test]
    fn no_subheadings() {
        let input = r#"## Heading 2
Hi everyone
## Heading 2"#;

        let expected_output = vec![
            vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2")),
                MarkdownSection::Unknown("Hi everyone"),
            ],
            vec![MarkdownSection::Heading(HeadingLevel::Top("## Heading 2"))],
        ];

        assert_eq!(divide_top_headings(input), expected_output);
    }

    #[wasm_bindgen_test]
    fn single_section() {
        let input = r#"## Heading 2
Hi everyone
### Subheading
Text under subheading"#;

        let expected_output = vec![vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2")),
            MarkdownSection::Unknown("Hi everyone"),
            MarkdownSection::Unknown("### Subheading"),
            MarkdownSection::Unknown("Text under subheading"),
        ]];

        assert_eq!(divide_top_headings(input), expected_output);
    }

    #[wasm_bindgen_test]
    fn empty_input() {
        let input = "";

        let expected_output: Vec<Vec<MarkdownSection>> = vec![];

        assert_eq!(divide_top_headings(input), expected_output);
    }
}
