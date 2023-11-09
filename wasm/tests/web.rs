//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// TODO: Update unit testing.

mod parse_input_test {
    use formatto_wasm::parsing_tools::get_section_vec;
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
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Unknown("Hi everyone".to_string()),
                MarkdownSection::Unknown("### Heading 3".to_string()),
                MarkdownSection::Unknown("end of heading 3".to_string()),
                MarkdownSection::Unknown("#### Heading 4".to_string()),
            ],
            vec![MarkdownSection::Heading(HeadingLevel::Top(
                "## Heading 2".to_string(),
            ))],
        ];

        assert_eq!(get_section_vec(input), expected_output);
    }

    #[wasm_bindgen_test]
    fn no_subheadings() {
        let input = r#"## Heading 2
Hi everyone
## Heading 2"#;

        let expected_output = vec![
            vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Unknown("Hi everyone".to_string()),
            ],
            vec![MarkdownSection::Heading(HeadingLevel::Top(
                "## Heading 2".to_string(),
            ))],
        ];

        assert_eq!(get_section_vec(input), expected_output);
    }

    #[wasm_bindgen_test]
    fn single_section() {
        let input = r#"## Heading 2
Hi everyone
### Subheading
Text under subheading"#;

        let expected_output = vec![vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Unknown("Hi everyone".to_string()),
            MarkdownSection::Unknown("### Subheading".to_string()),
            MarkdownSection::Unknown("Text under subheading".to_string()),
        ]];

        assert_eq!(get_section_vec(input), expected_output);
    }

    #[wasm_bindgen_test]
    fn empty_input() {
        let input = "";

        let expected_output: Vec<Vec<MarkdownSection>> = vec![];

        assert_eq!(get_section_vec(input), expected_output);
    }

    #[wasm_bindgen_test]
    fn code_block() {
        let input = r#"## Heading 2
Hi everyone

#### Heading 4
```rust
fn main(
    println!(\"Hello World\");
) {}
```
"#;

        let expected_output: Vec<Vec<MarkdownSection>> = vec![vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Unknown("Hi everyone".to_string()),
            MarkdownSection::Unknown("#### Heading 4".to_string()),
            MarkdownSection::Code(
                r#"```rust
fn main(
    println!(\"Hello World\");
) {}
```"#
                    .to_string(),
            ),
        ]];

        assert_eq!(get_section_vec(input), expected_output);
    }
}
