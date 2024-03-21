use onig::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Invalid args");
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file");

    loop {
        let commands = get_commands();

        if commands.operation == Operation::Find {
            find(&commands.find, &file_contents, &commands.output_file).expect("Failed to complete find");
        } else if commands.operation == Operation::Replace {
            file_contents = replace(&commands.find, &commands.replace, &file_contents).expect("Failed to complete replace");
        } else if commands.operation == Operation::Write {
            write(&file_contents, &commands.output_file).expect("Failed to write file");
        } else if commands.operation == Operation::Output {
            output(&file_contents).expect("Failed to output file");
        } else if commands.operation == Operation::Quit {
            return;
        }
    }
}

fn find(find_regex_string: &str, file_contents: &str, output_filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let find_regex_compiled = Regex::new(find_regex_string)?;
    let regex_match = find_regex_compiled.captures_iter(file_contents);
    let mut output_file: File = File::create(output_filepath)?;
    for captures in regex_match {
        let first_capture = captures.at(0).unwrap_or_default();
        output_file.write(format!("{}\n", first_capture).as_bytes())?;
    }
    Ok(())
}

fn replace(find_regex_string: &str, replace_string: &str, file_contents: &str) -> Result<String, Box<dyn std::error::Error>> {
    let find_regex_compiled = Regex::new(&find_regex_string)?;
    Ok(find_regex_compiled.replace_all(file_contents, replace_string))
}

fn write(file_contents: &str, output_filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_file: File = File::create(output_filepath)?;
    output_file.write(file_contents.as_bytes())?;
    Ok(())
}

fn output(file_contents: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", file_contents);
    Ok(())
}

fn get_commands() -> Commands {
    let input_string = read_line();
    let mut commands: Commands = Default::default();
    let input_split: Vec<&str> = input_string.split(" ").collect();
    match input_split.get(0).unwrap() {
        &"f" => commands.operation = Operation::Find,
        &"r" => commands.operation = Operation::Replace,
        &"w" => commands.operation = Operation::Write,
        &"o" => commands.operation = Operation::Output,
        &"q" => commands.operation = Operation::Quit,
        _ => commands.operation = Operation::None
    }

    match (input_split.len(), &commands.operation) {
        (3, Operation::Find) => {
            commands.find = input_split.get(1).unwrap_or(&"").to_string();
            commands.output_file = input_split.get(2).unwrap_or(&"").to_string();
        },
        (3, Operation::Replace) => {
            commands.find = input_split.get(1).unwrap_or(&"").to_string();
            commands.replace = input_split.get(2).unwrap_or(&"").to_string();
        },
        (2, Operation::Write) => {
            commands.output_file = input_split.get(1).unwrap_or(&"").to_string();
        },
        (1, Operation::Output | Operation::Quit) => {},
        (_, Operation::None) => {},
        (_, _) => {commands.operation = Operation::None; println!("Incorrect number of command params")}

    };
    return commands;
}

fn read_line() -> String {
    print!(":");
    std::io::Write::flush(&mut std::io::stdout()).unwrap_or_default();
    let mut buffer = String::default();
    std::io::stdin().read_line(&mut buffer).unwrap_or_default();
    let len = buffer.trim_end_matches(&['\r', '\n'][..]).len();
    buffer.truncate(len);
    return buffer.trim().to_string();
}

#[derive(Default)]
struct Commands {
    operation: Operation,
    find: String,
    replace: String,
    output_file: String
}

#[derive(PartialEq)]
pub enum Operation {
    None,
    Find,
    Replace,
    Write,
    Output,
    Quit
}

impl Default for Operation {
    fn default() -> Self { Operation::None }
}
