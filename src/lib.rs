use std::fs;
use unicode_segmentation::UnicodeSegmentation;

pub fn load_csv_file(filename: &String, delim: String, startline: u8, latex_command: String) -> String {
    let csv_file = fs::read_to_string(filename).expect("File not found.");
    let csv_file = csv_file.trim().to_string();
    let csv_file = csv_file.split("\n");
    let mut output_string: String = String::from("");
    for (index, line) in csv_file.enumerate() {
        if index < startline.into() {
            continue;
        }
        let datafields = line.split(&delim);
        let mut output_line: String = latex_command.clone();
        datafields.into_iter().for_each(|argument| {
            output_line.push_str("{");
            output_line.push_str(&trim_string(argument.to_string()));
            output_line.push_str("}");
        });
        output_string.push_str(&output_line);
        output_string.push_str("\n");
    }
    output_string.trim().to_string()
}

fn trim_string(s: String) -> String {
    let mut trimmed_string = s.trim().clone().to_string();
    let binding = trimmed_string.clone();
    let first_character = binding.graphemes(true).next();
    let trimmed_string_length = s.len();
    let binding = trimmed_string.clone();
    let last_character = binding.graphemes(true).last();
    if first_character == Some("\"") && last_character == Some("\"") {
        trimmed_string = trimmed_string[1..trimmed_string_length-2].to_string();
    }
    trimmed_string
}