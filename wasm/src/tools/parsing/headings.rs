/// Gets the top heading level of the entire document.
pub fn get_top_heading_level(input_lines: &[&str]) -> Option<usize> {
    use self::alternate_headings::get_valid_alternate_heading_level;
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

        // Parse alternate headings.
        let alternate_heading_level: Option<usize> =
            get_valid_alternate_heading_level(input_lines, index);

        if let Some(alternate_heading_level) = alternate_heading_level {
            if alternate_heading_level == 1 && 1 < top_heading_level {
                top_heading_level = 1;
            } else if alternate_heading_level == 2 && 2 < top_heading_level {
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
        /// Validates hash heading syntax.
        pub fn validate_hash_heading(line: &str) -> bool {
            line.starts_with('#') && (line.contains("# ") || line.chars().all(|char| char == '#'))
        }

        /// Validates hash heading syntax. (Top level heading)
        pub fn validate_top_hash_heading(line: &str, top_heading_hash_literal: &str) -> bool {
            line.starts_with(top_heading_hash_literal)
                && !line.starts_with(format!("{}#", top_heading_hash_literal).as_str())
        }
        /// Validates hash heading syntax. (Sub level heading)
        pub fn validate_sub_hash_heading(line: &str, only_contains_header_symbols: bool) -> bool {
            line.contains("# ") || only_contains_header_symbols
        }
    }
}

pub mod alternate_headings {
    /// Checks a valid alternate heading syntax and returns its level.
    pub fn get_valid_alternate_heading_level(
        input_lines: &[&str],
        reading_index: usize,
    ) -> Option<usize> {
        use validation::get_valid_alternate_top_heading_level::get_alternate_heading_level;
        use validation::validate_previous_alternate_headings;

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

        let heading_level = validate_previous_alternate_headings(input_lines, reading_index);
        if !heading_level {
            return None;
        }

        get_alternate_heading_level(input_lines[reading_index])
    }

    pub mod validation {
        pub fn validate_previous_alternate_headings(
            input_lines: &[&str],
            reading_index: usize,
        ) -> bool {
            use self::get_valid_alternate_top_heading_level::get_alternate_heading_level;

            let mut is_reading_syntax = true;
            let mut is_reading_title = false;

            for (index, &line) in input_lines[0..=reading_index].iter().enumerate().rev() {
                if line.is_empty() && is_reading_syntax && !is_reading_title {
                    return true;
                }

                if is_reading_syntax {
                    if super::super::hash_headings::validation::validate_hash_heading(line) {
                        return true;
                    }

                    if get_alternate_heading_level(line).is_some() {
                        is_reading_syntax = false;
                        is_reading_title = true;
                        continue;
                    }

                    return false;
                } else if is_reading_title {
                    if super::super::hash_headings::validation::validate_hash_heading(line) {
                        return false;
                    }
                    if index == 0 {
                        return true;
                    }

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

        pub mod get_valid_alternate_top_heading_level {
            /// Checks which level of alternate heading is being read.
            /// EXAMPLE: heading-1 or heading-2
            pub fn get_alternate_heading_level(line: &str) -> Option<usize> {
                if line.is_empty() {
                    return None;
                }

                let valid_alternate_heading_1 = line.chars().all(|char| char == '=');
                let valid_alternate_heading_2 = line.chars().all(|char| char == '-');

                if valid_alternate_heading_1 {
                    Some(1)
                } else if valid_alternate_heading_2 {
                    Some(2)
                } else {
                    None
                }
            }
        }

        /// Validates alternate top heading.
        pub fn validate_alternate_top_heading(
            lines: &[&str],
            reading_index: usize,
            top_heading_level: usize,
        ) -> bool {
            use super::get_valid_alternate_heading_level;

            let heading_level: Option<usize> =
                get_valid_alternate_heading_level(lines, reading_index);
            if let Some(heading_level) = heading_level {
                heading_level == top_heading_level
            } else {
                false
            }
        }
        /// Validates alternate sub heading.
        pub fn validate_alternate_sub_heading(
            lines: &[&str],
            reading_index: usize,
            top_heading_level: usize,
        ) -> bool {
            use super::get_valid_alternate_heading_level;

            let heading_level: Option<usize> =
                get_valid_alternate_heading_level(lines, reading_index);
            if let Some(heading_level) = heading_level {
                heading_level > top_heading_level
            } else {
                false
            }
        }
    }
}
