use onig::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = match args.get(1) {
        Some(some) => some,
        None => panic!("Invalid args"),
    };
    let mut file = match File::open(file_path) {
        Ok(ok) => ok,
        Err(_) => panic!("Failed to open file"),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(ok) => ok,
        Err(_) => panic!("Failed to read file"),
    };

    let commands = get_commands();
    let re = match Regex::new(&commands.find) {
        Ok(ok) => ok,
        Err(_) => panic!("Failed to compile regex"),
    };
    let match_test = re.captures_iter(&contents);

    let mut file = match File::create("output.txt") {
        Ok(ok) => ok,
        Err(_) => panic!("Failed to create output file"),
    };
    for captures in match_test {
        let first_capture = captures.at(0).unwrap_or_default();
        file.write(format!("{}\n", first_capture).as_bytes()).unwrap_or_default();
    }
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
