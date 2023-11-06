use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn status() -> bool {
    true
}

macro_rules! console_log {
    ($($arg:tt)*) => (log(&format!($($arg)*)));
}

pub fn divide_top_headings(input: &str) -> Vec<Vec<&str>> {
    let input_lines = input.trim().split('\n').collect::<Vec<&str>>();

    console_log!("{:#?}", &input_lines);

    //* Check top heading level.
    let top_heading_level = input_lines[0].chars().take_while(|&c| c == '#').count();
    let top_heading_sharp = "#".repeat(top_heading_level);

    console_log!("Top heading level: {}", top_heading_sharp);

    //* Dividing documents into sections
    let mut sections = Vec::<Vec<&str>>::new();
    let mut current_section = Vec::<&str>::new();

    for line in input_lines {
        let is_top_heading = line.starts_with(&top_heading_sharp)
            && !line.starts_with(format!("{}#", &top_heading_sharp).as_str());

        if is_top_heading && !current_section.is_empty() {
            sections.push(current_section);
            current_section = Vec::<&str>::new();
        }

        if !line.is_empty() {
            current_section.push(line);
        }
    }

    if !current_section.is_empty() {
        sections.push(current_section);
    }

    sections
}

#[wasm_bindgen]
pub fn format_document(input: &str) -> String {
    let top_heading_sections = divide_top_headings(input);

    // TODO

    input.to_string()
}
