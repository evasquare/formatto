/// Return the top heading level of a document.
pub fn get_top_heading_level(input_lines: &[&str]) -> Option<usize> {
    use crate::tools::parsing::headings::alternative_headings::get_alternative_heading_level;
    use crate::tools::parsing::headings::hash_headings::check_hash_heading_syntax;

    let mut top_heading_level: usize = usize::MAX;
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
        let valid_hash_heading = check_hash_heading_syntax(input_lines[index]);

        if valid_hash_heading {
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
    /// Validate hash heading syntax.
    pub fn check_hash_heading_syntax(line: &str) -> bool {
        line.starts_with('#') && (line.contains("# ") || line.chars().all(|char| char == '#'))
    }

    /// Validate hash heading syntax. (Top level heading)
    pub fn check_top_hash_heading(line: &str, top_heading_hash_literal: &str) -> bool {
        line.starts_with(top_heading_hash_literal)
            && !line.starts_with(format!("{}#", top_heading_hash_literal).as_str())
    }

    /// Validate hash heading syntax. (Sub level heading)
    pub fn check_sub_hash_heading(line: &str, only_contains_header_symbols: bool) -> bool {
        line.contains("# ") || only_contains_header_symbols
    }
}

pub mod alternative_headings {
    /// Check which level of alternative heading is being read.]
    /// (ex: heading-1 or heading-2)
    pub fn check_alternative_heading_level(line: &str) -> Option<usize> {
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

    /// Return the level of an alternative heading being read.
    pub fn get_alternative_heading_level(
        input_lines: &[&str],
        reading_index: usize,
    ) -> Option<usize> {
        use crate::tools::parsing::headings::hash_headings::check_hash_heading_syntax;

        if reading_index > input_lines.len() - 1 {
            return None;
        }
        if input_lines[reading_index].is_empty() {
            return None;
        }

        let previous_lines = {
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
            let third_line: Option<&str> = if reading_index > 2 {
                input_lines.get(reading_index - 3).copied()
            } else {
                None
            };

            (first_line, second_line, third_line)
        };

        let next_line: Option<&str> = if reading_index < input_lines.len() - 2 {
            input_lines.get(reading_index + 1).copied()
        } else {
            None
        };

        match (previous_lines.0, previous_lines.1, next_line) {
            (Some(previous_first_line), Some(previous_second_line), Some(_)) => {
                let valid_alternative_heading = (previous_second_line.is_empty()
                    || check_hash_heading_syntax(previous_second_line))
                    && !previous_first_line.is_empty();

                if !valid_alternative_heading {
                    return None;
                }

                check_alternative_heading_level(input_lines[reading_index])
            }
            (Some(previous_first_line), None, Some(_)) => {
                let valid_alternative_heading = !previous_first_line.is_empty();

                if !valid_alternative_heading {
                    return None;
                }

                check_alternative_heading_level(input_lines[reading_index])
            }
            (Some(previous_first_line), Some(previous_second_line), None) => {
                let valid_alternative_heading = (previous_second_line.is_empty()
                    || check_hash_heading_syntax(previous_second_line))
                    && !previous_first_line.is_empty();

                if !valid_alternative_heading {
                    return None;
                }

                check_alternative_heading_level(input_lines[reading_index])
            }
            (Some(previous_first_line), None, None) => {
                let valid_alternative_heading = !previous_first_line.is_empty();
                if !valid_alternative_heading {
                    return None;
                }

                check_alternative_heading_level(input_lines[reading_index])
            }
            _ => None,
        }
    }

    /// Validate alternative heading syntax. (Sub level heading)
    pub fn check_alternative_sub_heading(
        lines: &[&str],
        reading_index: usize,
        top_heading_level: usize,
    ) -> bool {
        let heading_level: Option<usize> = get_alternative_heading_level(lines, reading_index);

        if let Some(heading_level) = heading_level {
            heading_level > top_heading_level
        } else {
            false
        }
    }

    /// Validate alternative heading syntax. (Top level heading)
    pub fn check_alternative_top_heading(
        lines: &[&str],
        reading_index: usize,
        top_heading_level: usize,
    ) -> bool {
        let heading_level: Option<usize> = get_alternative_heading_level(lines, reading_index);

        if let Some(heading_level) = heading_level {
            heading_level == top_heading_level
        } else {
            false
        }
    }
}
