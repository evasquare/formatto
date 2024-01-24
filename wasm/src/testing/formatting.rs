#[cfg(test)]
mod get_formatted_string {
    use crate::{
        setting_schema::{FormatOptions, HeadingGaps, MainPluginSettings, OtherGaps, OtherOptions},
        testing::setup,
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
            format_options: FormatOptions {
                insert_newline: Some(false),
            },
            other_options: OtherOptions {
                notify_when_unchanged: Some(false),
            },
        }
    }

    #[test]
    fn multiple_headings() {
        setup();

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
        setup();

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
        setup();

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
        setup();

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
        setup();

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
