#[cfg(test)]
mod parse_input_test {
    use crate::{
        parsing_tools::get_section_vec,
        token_types::{HeadingLevel, MarkdownSection},
    };

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

        assert_eq!(get_section_vec(input), expected_output);
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

        assert_eq!(get_section_vec(input), expected_output);
    }

    #[test]
    fn single_section() {
        let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
### Subheading
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

        let expected_output = vec![
            MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
            MarkdownSection::Heading(HeadingLevel::FirstSub("### Subheading".to_string())),
            MarkdownSection::Content(
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                    .to_string(),
            ),
        ];

        assert_eq!(get_section_vec(input), expected_output);
    }

    #[test]
    fn empty_input() {
        let input = "";

        let expected_output = vec![];

        assert_eq!(get_section_vec(input), expected_output);
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
            MarkdownSection::Heading(HeadingLevel::Sub("#### Heading 4".to_string())),
            MarkdownSection::Code(
                r#"```rust
fn main(
    println!(\"Hello World\");
) {}
```"#
                    .to_string(),
            ),
        ];

        assert_eq!(get_section_vec(input), expected_output);
    }
}

#[cfg(test)]
mod get_top_heading_level {
    use crate::parsing_tools::get_top_heading_level;

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
