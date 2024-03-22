mod operations;
use operations::Operation;
mod commands;
mod args_parser;
use std::fs::File;
use std::io::prelude::*;
use onig::Regex;

fn main() {
    let args: args_parser::Args = args_parser::parse_args();
    let file_path = args.file;
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file");
    let mut previous_file_contents = String::new();
    let mut undo_available = false;

    loop {
        let commands = match commands::get_commands() {
            Some(some) => some,
            None => { println!("Unknown command or incorrect number of command params"); continue }
        };

        if args.undo && commands.destructive {
            previous_file_contents = file_contents.clone();
            undo_available = true;
        }

        let find_regex = match Regex::new(&commands.find) {
            Ok(ok) => ok,
            Err(err) => { println!("Failed to compile regex: {}", err); continue },
        };

        if commands.operation == Operation::Find {
            match operations::find(find_regex, &file_contents, &commands.output_file) {
                Ok(_) => {},
                Err(err) => { println!("Failed to complete find: {}", err); continue},
            };
        } else if commands.operation == Operation::Replace {
            file_contents = match operations::replace(find_regex, &commands.replace, &file_contents) {
                Ok(ok) => ok,
                Err(err) => { println!("Failed to complete replace: {}", err); continue},
            };
        } else if commands.operation == Operation::ReplaceWrite {
            file_contents = match operations::replace(find_regex, &commands.replace, &file_contents) {
                Ok(ok) => ok,
                Err(err) => { println!("Failed to complete replace: {}", err); continue},
            };
            match operations::write(&file_contents, &commands.output_file) {
                Ok(_) => {},
                Err(err) => { println!("Failed to write file: {}", err); continue},
            };
        } else if commands.operation == Operation::Write {
            match operations::write(&file_contents, &commands.output_file) {
                Ok(_) => {},
                Err(err) => { println!("Failed to write file: {}", err); continue},
            };
        } else if commands.operation == Operation::Output {
            match operations::output(&file_contents) {
                Ok(_) => {},
                Err(err) => { println!("Failed to output file: {}", err); continue},
            };
        } else if commands.operation == Operation::Undo {
            if !args.undo {
                println!("Undo is not enabled, pass the `--undo` arg to enable it");
                continue;
            }
            if !undo_available {
                println!("Failed to undo, file history not available");
                continue;
            }
            file_contents = previous_file_contents;
            previous_file_contents = String::new();
            undo_available = false;
        } else if commands.operation == Operation::Quit {
            return;
        }
    }
}
