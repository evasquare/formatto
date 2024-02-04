#[cfg(test)]
mod get_formatted_string {
    use crate::{
        testing::{get_example_settings, setup},
        tools::{formatting::get_formatted_string, parsing::get_sections},
    };

    #[test]
    fn alternative_headings_1() {
        setup();

        let input = r#"## Heading 2

### heading 3
```ts
console.log("Hello World");
```

Heading 1
==="#;
        let sections = get_sections(input, &get_example_settings()).unwrap();

        let output = get_formatted_string(sections, &get_example_settings()).unwrap();
        println!("{:#?}", output);
        let expected_output = r#"## Heading 2

### heading 3
```ts
console.log("Hello World");
```



Heading 1
==="#;

        assert_eq!(output, expected_output);
    }
    #[test]
    fn multiple_headings() {
        setup();

        let input = r#"## Heading 2
### Heading 3
#### Heading 4"#;
        let sections = get_sections(input, &get_example_settings()).unwrap();
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
        let sections = get_sections(input, &get_example_settings()).unwrap();
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

        let sections = get_sections(input, &get_example_settings()).unwrap();
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

        let sections = get_sections(input, &get_example_settings()).unwrap();
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

        let sections = get_sections(input, &get_example_settings()).unwrap();
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
