use crate::{
    testing::{get_example_locale, get_example_settings, setup},
    tools::{formatting::get_formatted_string, parsing::get_sections},
};

#[test]
fn case_1() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
```rust
fn main(
    println!(\"Hello World\");
) {}
```"#;

    let sections = get_sections(input, &get_example_settings(), &get_example_locale()).unwrap();
    let output =
        get_formatted_string(sections, &get_example_settings(), &get_example_locale()).unwrap();
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

#[test]
fn case_2() {
    setup();

    let input = r#"## Heading 2

```rust
fn main(
    println!(\"Hello World\");
) {}
```


Lorem Ipsum is simply dummy text of the printing and typesetting industry.
"#;

    let sections = get_sections(input, &get_example_settings(), &get_example_locale()).unwrap();
    let output =
        get_formatted_string(sections, &get_example_settings(), &get_example_locale()).unwrap();
    let expected_output = r#"## Heading 2
```rust
fn main(
    println!(\"Hello World\");
) {}
```

Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

    assert_eq!(output, expected_output);
}
