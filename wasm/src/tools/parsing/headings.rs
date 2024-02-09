/// Return the top heading level of a document.
pub fn get_top_heading_level(input_lines: &[&str]) -> Option<usize> {
    use self::alternative_headings::get_alternative_heading_level;
    use self::hash_headings::validation::validate_hash_heading;

    let mut top_heading_level = usize::MAX;
    let mut is_reading_code_block = false;

    for (index, &line) in input_lines.iter().enumerate() {
        // Skip code blocks.
        if line.starts_with("```") {
            is_reading_code_block = !is_reading_code_block;
        }
        if is_reading_code_block {
            continue;
        }

        // Parse hash headings.
        let is_valid_hash_heading = validate_hash_heading(input_lines[index]);

        if is_valid_hash_heading {
            let heading_level = line.chars().take_while(|&c| c == '#').count();
            if heading_level < top_heading_level {
                top_heading_level = heading_level;
            }

            if heading_level == 1 {
                break;
            }
        }

        // Parse alternative headings.
        let alternative_heading_level: Option<usize> =
            get_alternative_heading_level(input_lines, index);

        if let Some(alternative_heading_level) = alternative_heading_level {
            if alternative_heading_level == 1 && 1 < top_heading_level {
                top_heading_level = 1;
            } else if alternative_heading_level == 2 && 2 < top_heading_level {
                top_heading_level = 2;
            }
        }
    }

    if top_heading_level == usize::MAX {
        return None;
    }

    Some(top_heading_level)
}

pub mod hash_headings {
    pub mod validation {
        /// Validate hash heading syntax.
        pub fn validate_hash_heading(line: &str) -> bool {
            line.starts_with('#') && (line.contains("# ") || line.chars().all(|char| char == '#'))
        }

        /// Validate hash heading syntax. (Top level heading)
        pub fn validate_top_hash_heading(line: &str, top_heading_hash_literal: &str) -> bool {
            line.starts_with(top_heading_hash_literal)
                && !line.starts_with(format!("{}#", top_heading_hash_literal).as_str())
        }
        /// Validate hash heading syntax. (Sub level heading)
        pub fn validate_sub_hash_heading(line: &str, only_contains_header_symbols: bool) -> bool {
            line.contains("# ") || only_contains_header_symbols
        }
    }
}

pub mod alternative_headings {
    /// Get the level of an alternative heading being read.
    pub fn get_alternative_heading_level(
        input_lines: &[&str],
        reading_index: usize,
    ) -> Option<usize> {
        use validation::{
            get_valid_alternative_top_heading_level::get_valid_alternative_top_heading_level,
            validate_previous_alternative_headings,
        };

        if reading_index > input_lines.len() - 1 {
            return None;
        }
        if input_lines[reading_index].is_empty()
            || !input_lines[reading_index]
                .chars()
                .all(|char| char == '-' || char == '=')
        {
            return None;
        }

        if !validate_previous_alternative_headings(input_lines, reading_index) {
            return None;
        }

        get_valid_alternative_top_heading_level(input_lines, reading_index)
    }

    pub mod validation {
        pub fn validate_previous_alternative_headings(
            input_lines: &[&str],
            reading_index: usize,
        ) -> bool {
            use self::get_valid_alternative_top_heading_level::get_alternative_heading_level;

            let mut is_reading_syntax = true;
            let mut is_reading_title = false;

            for &line in input_lines[0..=reading_index].iter().rev() {
                if line.is_empty() && is_reading_syntax && !is_reading_title {
                    return true;
                }

                if is_reading_syntax {
                    if get_alternative_heading_level(line).is_some() {
                        is_reading_syntax = false;
                        is_reading_title = true;
                        continue;
                    }
                    return false;
                } else if is_reading_title {
                    if !line.is_empty() {
                        is_reading_syntax = true;
                        is_reading_title = false;
                        continue;
                    }

                    return false;
                }
            }

            false // Fallback.
        }

        pub mod get_valid_alternative_top_heading_level {
            pub fn get_valid_alternative_top_heading_level(
                input_lines: &[&str],
                reading_index: usize,
            ) -> Option<usize> {
                use super::super::super::hash_headings::validation::validate_hash_heading;

                let previous_lines = get_previous_lines(input_lines, reading_index);
                let next_line: Option<&str> = if reading_index < input_lines.len() - 2 {
                    input_lines.get(reading_index + 1).copied()
                } else {
                    None
                };

                match (previous_lines.0, previous_lines.1, next_line) {
                    (Some(previous_first_line), Some(previous_second_line), Some(_)) => {
                        let valid_alternative_heading = (previous_second_line.is_empty()
                            || validate_hash_heading(previous_second_line))
                            && !previous_first_line.is_empty();

                        if !valid_alternative_heading {
                            return None;
                        }

                        get_alternative_heading_level(input_lines[reading_index])
                    }
                    (Some(previous_first_line), None, Some(_)) => {
                        let valid_alternative_heading = !previous_first_line.is_empty();

                        if !valid_alternative_heading {
                            return None;
                        }

                        get_alternative_heading_level(input_lines[reading_index])
                    }
                    (Some(previous_first_line), Some(previous_second_line), None) => {
                        let valid_alternative_heading = (previous_second_line.is_empty()
                            || validate_hash_heading(previous_second_line))
                            && !previous_first_line.is_empty();

                        if !valid_alternative_heading {
                            return None;
                        }

                        get_alternative_heading_level(input_lines[reading_index])
                    }
                    (Some(previous_first_line), None, None) => {
                        let valid_alternative_heading = !previous_first_line.is_empty();
                        if !valid_alternative_heading {
                            return None;
                        }

                        get_alternative_heading_level(input_lines[reading_index])
                    }
                    _ => None,
                }
            }

            /// Check which level of alternative heading is being read.
            /// EXAMPLE: heading-1 or heading-2
            pub fn get_alternative_heading_level(line: &str) -> Option<usize> {
                if line.is_empty() {
                    return None;
                }

                let valid_alternative_heading_1 = line.chars().all(|char| char == '=');
                let valid_alternative_heading_2 = line.chars().all(|char| char == '-');

                if valid_alternative_heading_1 {
                    Some(1)
                } else if valid_alternative_heading_2 {
                    Some(2)
                } else {
                    None
                }
            }

            /// Get the last 2 lines before the currently reading line.
            fn get_previous_lines<'a>(
                input_lines: &'a [&str],
                reading_index: usize,
            ) -> (Option<&'a str>, Option<&'a str>) {
                let first_line: Option<&str> = if reading_index > 0 {
                    input_lines.get(reading_index - 1).copied()
                } else {
                    None
                };
                let second_line: Option<&str> = if reading_index > 1 {
                    input_lines.get(reading_index - 2).copied()
                } else {
                    None
                };

                (first_line, second_line)
            }
        }

        /// Validate alternative top heading.
        pub fn validate_alternative_top_heading(
            lines: &[&str],
            reading_index: usize,
            top_heading_level: usize,
        ) -> bool {
            use super::get_alternative_heading_level;

            let heading_level: Option<usize> = get_alternative_heading_level(lines, reading_index);
            if let Some(heading_level) = heading_level {
                heading_level == top_heading_level
            } else {
                false
            }
        }
        /// Validate alternative sub heading.
        pub fn validate_alternative_sub_heading(
            lines: &[&str],
            reading_index: usize,
            top_heading_level: usize,
        ) -> bool {
            use super::get_alternative_heading_level;

            let heading_level: Option<usize> = get_alternative_heading_level(lines, reading_index);
            if let Some(heading_level) = heading_level {
                heading_level > top_heading_level
            } else {
                false
            }
        }
    }
}
