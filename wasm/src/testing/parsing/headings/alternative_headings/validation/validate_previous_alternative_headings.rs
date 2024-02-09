use crate::tools::parsing::headings::alternative_headings::validation::validate_previous_alternative_headings;

#[test]
fn valid_syntax_1() {
    let input_lines: Vec<&str> = r#"## Heading 2

### Heading 3
```ts
console.log("Hello World");
```

Content
===
Content
---
Content
==="#
        .split('\n')
        .collect();

    let left = validate_previous_alternative_headings(&input_lines, 12);
    let right = true;

    assert_eq!(left, right);
}

#[test]
fn invalid_syntax_1() {
    let input_lines: Vec<&str> = r#"## Heading 2

### Heading 3
```ts
console.log("Hello World");
```

aaabbbccc
Content
===
Content
---
Content
==="#
        .split('\n')
        .collect();

    let left = validate_previous_alternative_headings(&input_lines, 12);
    let right = false;

    assert_eq!(left, right);
}
