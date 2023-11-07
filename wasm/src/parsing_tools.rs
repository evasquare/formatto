use crate::MarkdownSection;

pub fn parse_top_headings(input: &str) -> Vec<Vec<MarkdownSection>> {
    let input_lines: Vec<&str> = input.trim().split('\n').collect::<Vec<&str>>();

    //* Check top heading level.
    let top_heading_level = input_lines[0].chars().take_while(|&c| c == '#').count();
    let top_heading_sharp = "#".repeat(top_heading_level);

    //* Dividing documents into sections
    let mut sections = Vec::<Vec<MarkdownSection>>::new();
    let mut current_section = Vec::<MarkdownSection>::new();

    for line in input_lines {
        let is_top_heading = line.starts_with(&top_heading_sharp)
            && !line.starts_with(format!("{}#", &top_heading_sharp).as_str());

        if is_top_heading && !current_section.is_empty() {
            sections.push(current_section);
            current_section = Vec::<MarkdownSection>::new();
        }

        if !line.is_empty() {
            current_section.push(match is_top_heading {
                true => MarkdownSection::Heading(crate::HeadingLevel::Top(line)),
                false => MarkdownSection::Unknown(line),
            });
        }
    }

    // Push a vector of each top heading and its contents.
    if !current_section.is_empty() {
        sections.push(current_section);
    }

    sections
}
