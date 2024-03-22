mod operations;
mod macros;
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
            unwrap_or_continue!(operations::find(find_regex, &file_contents, &commands.output_file), "Failed to complete find");
        } else if commands.operation == Operation::Replace {
            file_contents = unwrap_or_continue!(operations::replace(find_regex, &commands.replace, &file_contents), "Failed to complete replace");
        } else if commands.operation == Operation::ReplaceWrite {
            file_contents = unwrap_or_continue!(operations::replace(find_regex, &commands.replace, &file_contents),"Failed to complete replace");
            unwrap_or_continue!(operations::write(&file_contents, &commands.output_file), "Failed to write file");
        } else if commands.operation == Operation::Write {
            unwrap_or_continue!(operations::write(&file_contents, &commands.output_file), "Failed to write file");
        } else if commands.operation == Operation::Output {
            unwrap_or_continue!(operations::output(&file_contents), "Failed to output file");
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
