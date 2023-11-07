use crate::MarkdownSection;

pub fn parse_top_headings(input: &str) -> Vec<Vec<MarkdownSection>> {
    let input_lines: Vec<&str> = input.trim().split('\n').collect::<Vec<&str>>();

    // Check top heading level.
    let top_heading_level = input_lines[0].chars().take_while(|&c| c == '#').count();
    let top_heading_sharp = "#".repeat(top_heading_level);

    // Dividing documents into sections
    let mut sections = Vec::<Vec<MarkdownSection>>::new();
    let mut current_section = Vec::<MarkdownSection>::new();

    let mut is_reading_code_block = false;
    let mut code_block = String::new();

    for line in input_lines {
        // Entering and exiting code block.
        if line.starts_with("```") && !is_reading_code_block {
            code_block.push_str(line);

            is_reading_code_block = true;
            continue;
        }
        if line.starts_with("```") && is_reading_code_block {
            code_block.push_str(format!("\n{}", line).as_str());
            current_section.push(MarkdownSection::Code(code_block.clone()));
            code_block.clear();

            is_reading_code_block = false;
            continue;
        }

        if is_reading_code_block {
            if !code_block.is_empty() {
                code_block.push('\n');
            }

            code_block.push_str(line);
            continue;
        }

        let is_top_heading = line.starts_with(&top_heading_sharp)
            && !line.starts_with(format!("{}#", &top_heading_sharp).as_str());

        if is_top_heading && !current_section.is_empty() {
            sections.push(current_section);
            current_section = Vec::<MarkdownSection>::new();
        }

        if !line.is_empty() {
            current_section.push(match is_top_heading {
                true => MarkdownSection::Heading(crate::HeadingLevel::Top(String::from(line))),
                false => MarkdownSection::Unknown(String::from(line)),
            });
        }
    }

    // Push a vector of each top heading and its contents.
    if !current_section.is_empty() {
        sections.push(current_section);
    }

    sections
}
