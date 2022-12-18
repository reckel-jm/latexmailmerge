use std::fs;
use clap::Parser;
use unicode_segmentation::UnicodeSegmentation;

fn main() {    
    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// The name of the LaTeX command which creates the the mail merge
        #[arg(long, default_value_t = String::from("\\merge"))]
        latex_command: String,
        /// First line to parse
        #[arg(short, long, default_value_t = 0)]
        firstline: u8,
        /// Delim
        #[arg(short, long, default_value_t = String::from(";"))]
        delim: String,
        /// Path of the CSV file
        filepath: String,
    }
    let options = Args::parse();

    load_csv_file(&options.filepath, options.delim, options.firstline, options.latex_command);
}

fn load_csv_file(filename: &String, delim: String, startline: u8, latex_command: String) {
    let csv_file = fs::read_to_string(filename).expect("File not found.");
    let csv_file = csv_file.trim().to_string();
    let csv_file = csv_file.split("\n");
    for (index, line) in csv_file.enumerate() {
        if index < startline.into() {
            continue;
        }
        let datafields = line.split(&delim);
        let mut output_line: String = latex_command.clone();
        for argument in datafields {
            output_line.push_str("{");
            output_line.push_str(&trim_string(argument.to_string()));
            output_line.push_str("}");
        }
        println!("{}", output_line);
    }
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