use crate::MarkdownSection;
use std::error::Error;

// TODO: Read other markdown sections too.

pub fn parse_input(input: &str) -> Result<String, Box<dyn Error>> {
    let input_lines: Vec<&str> = input.trim().split('\n').collect::<Vec<&str>>();

    let top_heading_level = input_lines[0].chars().take_while(|&c| c == '#').count();
    let top_heading_sharp = "#".repeat(top_heading_level);

    let mut sections = Vec::<Vec<MarkdownSection>>::new();
    let mut current_section = Vec::<MarkdownSection>::new();

    let mut code_block = String::new();
    let mut is_reading_code_block = false;

    for line in input_lines {
        // Parse code blocks.
        {
            // Entering and exiting a code block.
            if line.starts_with("```") && !is_reading_code_block {
                code_block.push_str(line);

                is_reading_code_block = true;
                continue;
            }
            if line.starts_with("```") && is_reading_code_block {
                code_block.push_str(format!("\n{}", line).as_str());
                current_section.push(MarkdownSection::Code(code_block.clone()));

                // Clear the temporary code block.
                code_block.clear();

                is_reading_code_block = false;
                continue;
            }

            // Keep reading the code block.
            if is_reading_code_block {
                if !code_block.is_empty() {
                    code_block.push('\n');
                }

                code_block.push_str(line);
                continue;
            }
        }

        // Parse headings.
        {
            let is_top_heading = line.starts_with(&top_heading_sharp)
                && !line.starts_with(format!("{}#", &top_heading_sharp).as_str());

            if is_top_heading && !current_section.is_empty() {
                sections.push(current_section);
                current_section = Vec::<MarkdownSection>::new();
            }

            // TODO: Parse sub-headings.

            if !line.is_empty() {
                current_section.push(match is_top_heading {
                    true => MarkdownSection::Heading(crate::HeadingLevel::Top(line.to_string())),
                    false => MarkdownSection::Unknown(line.to_string()),
                });
            }
        }
    }

    // Push a vector of each top heading and its contents.
    if !current_section.is_empty() {
        sections.push(current_section);
    }

    // TODO: Replace this with the formatted string.
    Ok(String::new())
}
