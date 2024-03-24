use std::{fs::File, io::Write};
use onig::Regex;

pub fn find(find_regex_compiled: &Regex, file_contents: &str, mut output_file: &File) -> Result<(), Box<dyn std::error::Error>> {
    let regex_match = find_regex_compiled.captures_iter(file_contents);
    for captures in regex_match {
        let first_capture = captures.at(0).unwrap_or_default();
        output_file.write(format!("{}\n", first_capture).as_bytes())?;
    }
    Ok(())
}

pub fn replace(find_regex_compiled: &Regex, replace_string: &str, file_contents: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(find_regex_compiled.replace_all(file_contents, replace_string))
}

pub fn write(file_contents: &str, mut output_file: &File) -> Result<(), Box<dyn std::error::Error>> {
    output_file.write(file_contents.as_bytes())?;
    Ok(())
}

pub fn output(file_contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", file_contents);
    Ok(())
}

#[derive(PartialEq)]
pub enum Operation {
    None,
    Find,
    Replace,
    ReplaceActive,
    Write,
    Output,
    Undo,
    Quit
}

impl Default for Operation {
    fn default() -> Self { Operation::None }
}
