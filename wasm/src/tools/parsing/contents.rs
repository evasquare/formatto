use super::super::tokens::MarkdownSection;

/// Finishes the current "content" section and pushes it into the "sections" vector.
pub fn finish_current_content_section(
    is_reading_content_section: &mut bool,
    sections: &mut Vec<MarkdownSection>,
    temp_content_section: &mut String,
) {
    *is_reading_content_section = false;

    // Check if the current "content" is empty.
    // Because this function will also be called with empty values.
    if temp_content_section.is_empty() {
        return;
    }

    sections.push(MarkdownSection::Content(
        temp_content_section.trim_end().to_string(),
    ));
    temp_content_section.clear();
}

/// Appends a line with a line break.
pub fn append_a_line_break(string: &mut String, line: &str) {
    if !string.is_empty() {
        string.push('\n');
    }

    string.push_str(line);
}
