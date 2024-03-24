mod operations;
mod macros;
use operations::Operation;
mod commands;
mod args_parser;
mod string_parse;
mod kuuubediterror;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: args_parser::Args = args_parser::parse_args();
    let file_path = &args.file;

    let mut file = File::open(file_path).expect("Failed to open file");

    let file_size = file.metadata().unwrap().len();
    let buffer_size: u64 = if args.no_buf || args.buffer > file_size {
        file_size
    } else {
        args.buffer
    };

    let mut file_buffer = vec![0u8; buffer_size as usize];

    let mut file_contents = if args.no_buf {
        let mut string_buffer = String::default();
        file.read_to_string(&mut string_buffer).unwrap();
        string_buffer
    } else {
        String::default()
    };
    let mut previous_file_contents: Option<String> = None;

    loop {
        let mut commands = unwrap_result_or_continue!(commands::get_commands(&args), "");

        file_buffer.clear();
        unwrap_result_or_continue!(file.seek(std::io::SeekFrom::Start(0)), "Failed to reset file stream position");

        if args.undo && commands.destructive {
            previous_file_contents = Some(file_contents.clone());
        }

        let mut end_loop = if args.no_buf { true } else { false };

        loop {
            if commands.operation == Operation::Quit {
                return;
            }

            if !args.no_buf {
                if unwrap_result_or_break!(file.stream_position(), "Failed to get stream position") + buffer_size > file_size {
                    file_buffer.clear();
                    unwrap_result_or_break!(file.read_to_end(&mut file_buffer), "Failed to read last file chunk");
                    end_loop = true;
                } else {
                    unwrap_result_or_break!(file.read(&mut file_buffer), "Failed to read file chunk");
                }
                file_contents = match std::str::from_utf8(&file_buffer) {
                    Ok(ok) => { ok.to_string() },
                    Err(err) => {
                        unwrap_result_or_break!(file.seek(std::io::SeekFrom::Current(buffer_size as i64 * -1)), "Failed to seek in file");
                        file_buffer.resize(err.valid_up_to(), b'\x00');
                        unwrap_result_or_break!(file.read_exact(&mut file_buffer), "Failed to re-read file chunk");
                        std::str::from_utf8(&file_buffer).unwrap_or({
                            println!("Invalid UTF-8 detected, output may not match input: {}", err);
                            unsafe { std::str::from_utf8_unchecked(&file_buffer) }
                        }).to_string()
                    },
                };
            }

            if commands.operation == Operation::Find {
                commands.output_file = unwrap_result_or_break!(operations::find(&commands.find_regex, &file_contents, commands.output_file), "Failed to write file");
            } else if commands.operation == Operation::Replace {
                file_contents = unwrap_result_or_break!(operations::replace(&commands.find_regex, &commands.replace, &file_contents),"Failed to complete replace");
                commands.output_file = unwrap_result_or_break!(operations::write(&file_contents, commands.output_file), "Failed to write file");
            } else if commands.operation == Operation::Write {
                commands.output_file = unwrap_result_or_break!(operations::write(&file_contents, commands.output_file), "Failed to write file");
            }
            //--no-buf only
            else if commands.operation == Operation::ReplaceActive {
                file_contents = unwrap_result_or_break!(operations::replace(&commands.find_regex, &commands.replace, &file_contents), "Failed to complete replace");
            } else if commands.operation == Operation::Output {
                unwrap_result_or_break!(operations::output(&file_contents), "Failed to output file");
            } else if commands.operation == Operation::Undo {
                file_contents = unwrap_option_or_break!(previous_file_contents, "Failed to undo, file history not available");
                previous_file_contents = None;
            }

            if end_loop { break; }

            file_buffer.resize(buffer_size as usize, b'\x00');
        }
    }
}
