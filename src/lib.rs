use std::ffi::c_uint;
use std::fs;
use std::ffi::{CString,CStr};
use std::os::raw::c_char;
use std::str;
use unicode_segmentation::UnicodeSegmentation;

/// Exports a Convertion function for DLL/so libraries
#[no_mangle]
pub extern "C" fn convert_csv_file(c_str_filename: *const c_char, c_str_delim: *const c_char, c_uint_startline: c_uint, c_str_latex_command: *const c_char) -> *mut c_char {
    let filename: &CStr = unsafe {
        CStr::from_ptr(c_str_filename)
    };
    let filename: String = filename.to_str().unwrap().to_owned();
    let delim: &CStr = unsafe {
        CStr::from_ptr(c_str_delim)
    };
    let delim: String = delim.to_str().unwrap().to_owned();
    let startline: u8 = c_uint_startline.try_into().unwrap();
    let latex_command: &CStr = unsafe { CStr::from_ptr(c_str_latex_command) };
    let latex_command: String = latex_command.to_str().unwrap().to_owned();
    
    let result = load_csv_file(&filename, &delim, &startline, &latex_command);
    let result: CString = CString::new(&result[..]).unwrap();

    result.into_raw()
}

pub fn load_csv_file(filename: &String, delim: &String, startline: &u8, latex_command: &String) -> String {
    let csv_file = fs::read_to_string(filename).expect("File not found.");
    let csv_file = csv_file.trim().to_string();
    let csv_file = csv_file.split("\n");
    let mut output_string: String = String::from("");
    let startline: u8 = startline.to_owned();
    for (index, line) in csv_file.enumerate() {
        if index < startline.into() {
            continue;
        }
        let datafields = line.split(delim);
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