use std::{fs::File, io::Write};
use onig::Regex;
use crate::kuuubediterror::KuuubeditError;

pub fn find(find_regex_compiled: &Regex, file_contents: &str, output_file_option: Option<File>) -> Result<Option<File>, Box<dyn std::error::Error>> {
    let mut output_file = output_file_option.ok_or(KuuubeditError::OutputFileNone)?;
    let regex_match = find_regex_compiled.captures_iter(file_contents);
    for captures in regex_match {
        let first_capture = captures.at(0).unwrap_or_default();
        output_file.write(format!("{}\n", first_capture).as_bytes())?;
    }
    Ok(Some(output_file))
}

pub fn replace(find_regex_compiled: &Regex, replace_string: &str, file_contents: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(find_regex_compiled.replace_all(file_contents, replace_string))
}

pub fn write(file_contents: &str, output_file_option: Option<File>) -> Result<Option<File>, Box<dyn std::error::Error>> {
    let mut output_file = output_file_option.ok_or(KuuubeditError::OutputFileNone)?;
    output_file.write(file_contents.as_bytes())?;
    Ok(Some(output_file))
}

pub fn output(file_contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", file_contents);
    Ok(())
}

#[derive(PartialEq)]
pub enum Operation {
    Find,
    Replace,
    ReplaceActive,
    Write,
    View,
    Output,
    Undo,
    Quit
}

impl Default for Operation {
    fn default() -> Self { Operation::Quit }
}
