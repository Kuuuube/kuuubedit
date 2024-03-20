use onig::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Invalid args");
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let commands = get_commands();

    let output_filepath = "output.txt";

    match commands.operation {
        Operation::Find => find(&commands.find,&contents, output_filepath).expect("Failed to complete find"),
        Operation::Replace => replace(&commands.find, &commands.replace, &contents, output_filepath).expect("Failed to complete replace"),
        _ => return
    }
}

fn find(find_regex_string: &str, input_string: &str, output_filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let find_regex_compiled = Regex::new(find_regex_string)?;
    let regex_match = find_regex_compiled.captures_iter(input_string);
    let mut output_file: File = File::create(output_filepath)?;
    for captures in regex_match {
        let first_capture = captures.at(0).unwrap_or_default();
        output_file.write(format!("{}\n", first_capture).as_bytes())?;
    }
    println!("Find completed successfully");
    Ok(())
}

fn replace(find_regex_string: &str, replace_string: &str, input_string: &str, output_filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let find_regex_compiled = Regex::new(&find_regex_string)?;
    let regex_replace = find_regex_compiled.replace_all(input_string, replace_string);
    let mut output_file: File = File::create(output_filepath)?;
    output_file.write(regex_replace.as_bytes())?;
    println!("Replace completed successfully");
    Ok(())
}

fn get_commands() -> Commands {
    let input_string = read_line();
    let mut commands: Commands = Default::default();
    let input_split: Vec<&str> = input_string.split(" ").collect();
    match input_split.get(0).unwrap() {
        &"f" => commands.operation = Operation::Find,
        &"r" => commands.operation = Operation::Replace,
        _ => commands.operation = Operation::None
    }

    match (input_split.len(), &commands.operation) {
        (2, Operation::Find) => {
            commands.find = input_split.get(1).unwrap_or(&"").to_string();
        }
        (3, Operation::Replace) => {
            commands.find = input_split.get(1).unwrap_or(&"").to_string();
            commands.replace = input_split.get(2).unwrap_or(&"").to_string();
        }
        (_, _) => panic!("Incorrect number of command params")

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
    return buffer;
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
    Replace
}

impl Default for Operation {
    fn default() -> Self { Operation::None }
}
