mod parsing_tools {
    #[cfg(test)]
    mod get_top_heading_level {
        use crate::parsing::parsing_tools::get_top_heading_level;

        #[test]
        fn multiple_headings() {
            let input: Vec<&str> = r#"## Heading 2
### Heading 3
#### Heading 4"#
                .split('\n')
                .collect();

            let output = get_top_heading_level(&input);
            let expected_output = 2;

            assert_eq!(output, expected_output);
        }

        #[test]
        fn content_before_headings() {
            let input: Vec<&str> =
                r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry.
## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 2"#
                    .split('\n')
                    .collect();

            let output = get_top_heading_level(&input);
            let expected_output = 2;

            assert_eq!(output, expected_output);
        }

        #[test]
        fn without_subheadings() {
            let input: Vec<&str> = r#"
## Heading 2
## Heading 2"#
                .split('\n')
                .collect();

            let output = get_top_heading_level(&input);
            let expected_output = 2;

            assert_eq!(output, expected_output);
        }

        #[test]
        fn top_heading_at_start_of_input() {
            let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
```rust
fn main(
    println!(\"Hello World\");
) {}
```"#
                .split('\n')
                .collect::<Vec<&str>>();
            let expected_output = 2;

            assert_eq!(get_top_heading_level(&input), expected_output);
        }

        #[test]
        fn top_heading_in_middle_of_input() {
            let input = r#"#### Heading 4
        
## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
```rust
fn main(
    println!(\"Hello World\");
) {}
```"#
                .split('\n')
                .collect::<Vec<&str>>();
            let expected_output = 2;

            assert_eq!(get_top_heading_level(&input), expected_output);
        }

        #[test]
        fn top_heading_at_end_of_input() {
            let input = r#"#### Heading 4
        
## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
```rust
fn main(
    println!(\"Hello World\");
) {}
```

# Heading 1
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#
                .split('\n')
                .collect::<Vec<&str>>();
            let expected_output = 1;

            assert_eq!(get_top_heading_level(&input), expected_output);
        }
    }

    #[cfg(test)]
    mod get_sections {
        use crate::{
            parsing::parsing_tools::get_sections,
            types::token_types::{HeadingLevel, MarkdownSection},
        };

        #[test]
        fn non_headings() {
            let input = r#"##Heading 2
###Heading 3
####Heading 4"#;

            let expected_output = vec![MarkdownSection::Content(
                r#"##Heading 2
###Heading 3
####Heading 4"#
                    .to_string(),
            )];

            assert_eq!(get_sections(input), expected_output);
        }

        #[test]
        fn only_headings_1() {
            let input = r#"## Heading 2
## Heading 2
## Heading 2"#;

            let expected_output = vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            ];

            assert_eq!(get_sections(input), expected_output);
        }

        #[test]
        fn only_headings_2() {
            let input = r#"## Heading 2
### Heading 3
#### Heading 4"#;

            let expected_output = vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
                MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
            ];

            assert_eq!(get_sections(input), expected_output);
        }

        #[test]
        fn only_content() {
            let input = r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

            let expected_output = vec![MarkdownSection::Content(
                r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#
                    .to_string(),
            )];

            assert_eq!(get_sections(input), expected_output);
        }

        #[test]
        fn sub_heading() {
            let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

            let expected_output = vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
                MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
                MarkdownSection::Heading(HeadingLevel::Sub("### Heading 3".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
            ];

            assert_eq!(get_sections(input), expected_output);
        }

        #[test]
        fn properties() {
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

            let expected_output = vec![
                MarkdownSection::Property("---\naliases:\n  - Test\n---".to_string()),
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
                MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
                MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            ];

            assert_eq!(get_sections(input), expected_output);
        }

        #[test]
        fn random_line_breaks() {
            let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3

Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
## Heading 2"#;

            let expected_output = vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
                MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
                MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            ];

            assert_eq!(get_sections(input), expected_output);
        }

        #[test]
        fn no_subheadings() {
            let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
## Heading 2"#;

            let expected_output = vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            ];

            assert_eq!(get_sections(input), expected_output);
        }

        #[test]
        fn two_headings_with_each_content() {
            let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

            let expected_output = vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
                MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
            ];

            assert_eq!(get_sections(input), expected_output);
        }

        #[test]
        fn empty_input() {
            let input = "";

            let expected_output = vec![];

            assert_eq!(get_sections(input), expected_output);
        }

        #[test]
        fn code_block() {
            let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
```rust
fn main(
    println!(\"Hello World\");
) {}
```"#;

            let expected_output = vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
                MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
                MarkdownSection::Code(
                    r#"```rust
fn main(
    println!(\"Hello World\");
) {}
```"#
                        .to_string(),
                ),
            ];

            assert_eq!(get_sections(input), expected_output);
        }
    }
}
