mod operations;
mod macros;
use operations::Operation;
mod commands;
mod args_parser;
use std::fs::File;
use std::os::unix::prelude::FileExt;
use onig::Regex;

fn main() {
    let args: args_parser::Args = args_parser::parse_args();
    let file_path = args.file;

    let file = File::open(file_path).expect("Failed to open file");

    let file_size = file.metadata().unwrap().len();
    let buffer_size: u64 = if args.no_buf || args.buffer > file_size {
        file_size
    } else {
        args.buffer
    };

    let mut file_buffer = vec![0u8; buffer_size as usize];

    let mut file_contents = if args.no_buf {
        file.read_at(&mut file_buffer, 0).unwrap();
        std::str::from_utf8(&file_buffer).unwrap().to_string()
    } else {
        String::default()
    };
    let mut previous_file_contents: Option<String> = None;

    loop {
        let mut file_offset = 0;

        let commands = match commands::get_commands() {
            Some(some) => some,
            None => { println!("Invalid command or incorrect number of command params"); continue }
        };

        if !args.no_buf && commands.no_buffer {
            println!("This command is only available when the `--no-buf` arg is passed");
            continue;
        }

        let find_regex = match Regex::new(&commands.find) {
            Ok(ok) => ok,
            Err(err) => { println!("Failed to compile regex: {}", err); continue },
        };

        let output_file = File::create(commands.output_file);

        while file_offset < file_size {
            if args.undo && commands.destructive {
                previous_file_contents = Some(file_contents.clone());
            }

            //clear buffer and trim string on last chunk of file
            if !args.no_buf {
                if file_offset + buffer_size > file_size {
                    file_buffer = vec![0u8; buffer_size as usize];
                    file.read_at(&mut file_buffer, file_offset).unwrap();
                    file_contents = std::str::from_utf8(&file_buffer).unwrap().trim_end_matches('\x00').to_string();
                } else {
                    file.read_at(&mut file_buffer, file_offset).unwrap();
                    //this fails when encountering multi-byte characters split on the buffer boundary that are invalid unicode when alone
                    file_contents = std::str::from_utf8(&file_buffer).unwrap().to_string();
                }
            }

            if commands.operation == Operation::Find {
                unwrap_result_or_continue!(operations::find(&find_regex, &file_contents, unwrap_result_or_continue!(&output_file, "Failed to create output file")), "Failed to complete find");
            } else if commands.operation == Operation::Replace {
                file_contents = unwrap_result_or_continue!(operations::replace(&find_regex, &commands.replace, &file_contents), "Failed to complete replace");
            } else if commands.operation == Operation::ReplaceWrite {
                file_contents = unwrap_result_or_continue!(operations::replace(&find_regex, &commands.replace, &file_contents),"Failed to complete replace");
                unwrap_result_or_continue!(operations::write(&file_contents, unwrap_result_or_continue!(&output_file, "Failed to create output file")), "Failed to write file");
            } else if commands.operation == Operation::Write {
                unwrap_result_or_continue!(operations::write(&file_contents, unwrap_result_or_continue!(&output_file, "Failed to create output file")), "Failed to write file");
            } else if commands.operation == Operation::Output {
                unwrap_result_or_continue!(operations::output(&file_contents), "Failed to output file");
            } else if commands.operation == Operation::Undo {
                if !args.undo {
                    println!("Undo is only available when the `--undo` arg is passed");
                    continue;
                }
                file_contents = unwrap_option_or_continue!(previous_file_contents, "Failed to undo, file history not available");
                previous_file_contents = None;
            } else if commands.operation == Operation::Quit {
                return;
            }

            file_offset += buffer_size;
        }
    }
}
