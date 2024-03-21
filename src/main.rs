mod operations;
use operations::Operation;
mod commands;
mod args_parser;
use std::fs::File;
use std::io::prelude::*;

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

        if commands.operation == Operation::Find {
            operations::find(&commands.find, &file_contents, &commands.output_file).expect("Failed to complete find");
        } else if commands.operation == Operation::Replace {
            file_contents = operations::replace(&commands.find, &commands.replace, &file_contents).expect("Failed to complete replace");
        } else if commands.operation == Operation::ReplaceWrite {
            file_contents = operations::replace(&commands.find, &commands.replace, &file_contents).expect("Failed to complete replace");
            operations::write(&file_contents, &commands.output_file).expect("Failed to write file");
        } else if commands.operation == Operation::Write {
            operations::write(&file_contents, &commands.output_file).expect("Failed to write file");
        } else if commands.operation == Operation::Output {
            operations::output(&file_contents).expect("Failed to output file");
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
