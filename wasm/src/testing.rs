mod formatting {
    #[cfg(test)]
    mod get_formatted_string {
        use crate::{
            setting_schema::{HeadingGaps, MainPluginSettings, OtherGaps, AdditionalSettings},
            tools::{formatting::get_formatted_string, parsing::get_sections},
        };

        fn get_example_settings() -> MainPluginSettings {
            MainPluginSettings {
                heading_gaps: HeadingGaps {
                    before_top_level_headings: Some("3".to_string()),
                    before_first_sub_heading: Some("1".to_string()),
                    before_sub_headings: Some("2".to_string()),
                },
                other_gaps: OtherGaps {
                    after_properties: Some("2".to_string()),
                    before_contents: Some("0".to_string()),
                    before_contents_after_code_blocks: Some("1".to_string()),
                    before_code_blocks: Some("1".to_string()),
                    before_code_blocks_after_headings: Some("0".to_string()),
                },
                additional_settings: AdditionalSettings {
                    add_empty_line: Some(false),
                }
            }
        }

        #[test]
        fn multiple_headings() {
            let input = r#"## Heading 2
### Heading 3
#### Heading 4"#;
            let sections = get_sections(input).unwrap();
            let output = get_formatted_string(sections, &get_example_settings()).unwrap();
            let expected_output = r#"## Heading 2

### Heading 3

#### Heading 4"#;

            assert_eq!(output, expected_output);
        }

        #[test]
        fn non_headings() {
            let input = r#"##Heading 2
###Heading 3
####Heading 4"#;
            let sections = get_sections(input).unwrap();
            let output = get_formatted_string(sections, &get_example_settings()).unwrap();
            let expected_output = r#"##Heading 2
###Heading 3
####Heading 4"#;

            assert_eq!(output, expected_output);
        }

        #[test]
        fn properties_1() {
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

            let sections = get_sections(input).unwrap();
            let output = get_formatted_string(sections, &get_example_settings()).unwrap();
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
        fn properties_2() {
            let input = r#"---
aliases:
  - Test
---"#;

            let sections = get_sections(input).unwrap();
            let output = get_formatted_string(sections, &get_example_settings()).unwrap();
            let expected_output = r#"---
aliases:
  - Test
---"#;

            assert_eq!(output, expected_output);
        }

        #[test]
        fn code_blocks() {
            let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
```rust
fn main(
    println!(\"Hello World\");
) {}
```"#;

            let sections = get_sections(input).unwrap();
            let output = get_formatted_string(sections, &get_example_settings()).unwrap();
            let expected_output = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
```rust
fn main(
    println!(\"Hello World\");
) {}
```"#;

            assert_eq!(output, expected_output);
        }
    }
}

mod parsing {
    #[cfg(test)]
    mod get_top_heading_level {
        use crate::tools::parsing::get_top_heading_level;

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
        use crate::tools::{
            parsing::get_sections,
            tokens::{HeadingLevel, MarkdownSection},
        };

        #[test]
        fn invalid_input() {
            let input = r#"```
code
SPACE```"#;
            let sections = get_sections(input);
            assert!(sections.is_err());
        }

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

            assert_eq!(get_sections(input).unwrap(), expected_output);
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

            assert_eq!(get_sections(input).unwrap(), expected_output);
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

            assert_eq!(get_sections(input).unwrap(), expected_output);
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

            assert_eq!(get_sections(input).unwrap(), expected_output);
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

            assert_eq!(get_sections(input).unwrap(), expected_output);
        }

        #[test]
        fn properties_1() {
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

            assert_eq!(get_sections(input).unwrap(), expected_output);
        }

        #[test]
        fn properties_2() {
            let input = r#"---
aliases:
  - Test
---
"#;

            let expected_output = vec![MarkdownSection::Property(
                "---\naliases:\n  - Test\n---".to_string(),
            )];

            assert_eq!(get_sections(input).unwrap(), expected_output);
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

            assert_eq!(get_sections(input).unwrap(), expected_output);
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

            assert_eq!(get_sections(input).unwrap(), expected_output);
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

            assert_eq!(get_sections(input).unwrap(), expected_output);
        }

        #[test]
        fn empty_input() {
            let input = "";

            let expected_output = vec![];

            assert_eq!(get_sections(input).unwrap(), expected_output);
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

            assert_eq!(get_sections(input).unwrap(), expected_output);
        }

        #[test]
        fn headings_without_letters() {
            let input = r#"#
##
##
##
##
###
###
####
####
##
#
"#;

            let expected_output = vec![
                MarkdownSection::Heading(HeadingLevel::Top("#".to_string())),
                MarkdownSection::Heading(HeadingLevel::FirstSub("##".to_string())),
                MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
                MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
                MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
                MarkdownSection::Heading(HeadingLevel::FirstSub("###".to_string())),
                MarkdownSection::Heading(HeadingLevel::Sub("###".to_string())),
                MarkdownSection::Heading(HeadingLevel::FirstSub("####".to_string())),
                MarkdownSection::Heading(HeadingLevel::Sub("####".to_string())),
                MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
                MarkdownSection::Heading(HeadingLevel::Top("#".to_string())),
            ];

            assert_eq!(get_sections(input).unwrap(), expected_output);
        }

        #[test]
        fn contents_with_line_breaks() {
            let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

\
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
"#;

            let expected_output = vec![
                MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
                MarkdownSection::Content(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                        .to_string(),
                ),
                MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
                MarkdownSection::Content(
                    r#"Lorem Ipsum is simply dummy text of the printing and typesetting industry.

\
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#
                        .to_string(),
                ),
            ];

            assert_eq!(get_sections(input).unwrap(), expected_output);
        }
    }
}
