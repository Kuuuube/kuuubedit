use crate::operations::Operation;
use crate::string_parse::*;
use crate::kuuubediterror::KuuubeditError;
use std::fs::File;
use std::path::Path;

pub fn get_commands(args: &crate::args_parser::Args) -> Result<Commands, Box<dyn std::error::Error>> {
    let input_string = read_line();
    let mut commands: Commands = Default::default();
    let input_split: Vec<String> = parse_command_string(&input_string);
    match input_split.get(0).ok_or(KuuubeditError::CommandParams)?.as_str() {
        "f" => commands.operation = Operation::Find,
        "r" => commands.operation = Operation::Replace,
        "ra" => commands.operation = Operation::ReplaceActive,
        "w" => commands.operation = Operation::Write,
        "o" => commands.operation = Operation::Output,
        "u" => commands.operation = Operation::Undo,
        "q" => commands.operation = Operation::Quit,
        "v" => commands.operation = Operation::View,
        "h" | "help" => commands.operation = Operation::Help,
        _ => commands.operation = Err(KuuubeditError::CommandName)?
    }

    match &commands.operation {
        Operation::Find | Operation::Replace | Operation::Write | Operation::Quit | Operation::View | Operation::Help => {}
        Operation::ReplaceActive | Operation::Output | Operation::Undo => if !args.no_buf { Err(KuuubeditError::CommandBuffer)? },
    }

    if commands.operation == Operation::Undo && !args.undo {
        Err(KuuubeditError::CommandUndo)?
    }

    match &commands.operation {
        Operation::Find => {
            commands.find_regex = onig::Regex::new(input_split.get(1).ok_or(KuuubeditError::CommandParams)?)?;
            commands.output_file = Some(get_output_file(&args.file, input_split.get(2).ok_or(KuuubeditError::CommandParams)?)?);
        },
        Operation::Replace => {
            commands.find_regex = onig::Regex::new(input_split.get(1).ok_or(KuuubeditError::CommandParams)?)?;
            commands.replace = unescape(input_split.get(2).ok_or(KuuubeditError::CommandParams)?)?;
            commands.output_file = Some(get_output_file(&args.file, input_split.get(3).ok_or(KuuubeditError::CommandParams)?)?);
            commands.destructive = true;
        },
        Operation::Write => {
            commands.output_file = Some(get_output_file(&args.file, input_split.get(1).ok_or(KuuubeditError::CommandParams)?)?);
        },
        Operation::View => {
            commands.view_start = u64::from_str_radix(input_split.get(1).ok_or(KuuubeditError::CommandParams)?, 10)?;
            commands.view_length = usize::from_str_radix(input_split.get(2).ok_or(KuuubeditError::CommandParams)?, 10)?;
        }
        Operation::ReplaceActive => {
            commands.find_regex = onig::Regex::new(input_split.get(1).ok_or(KuuubeditError::CommandParams)?)?;
            commands.replace = unescape(input_split.get(2).ok_or(KuuubeditError::CommandParams)?)?;
            commands.destructive = true;
        },
        Operation::Output | Operation::Undo | Operation::Quit | Operation::Help => {}
    };
    return Ok(commands);
}

fn get_output_file(input_filepath: &str, output_filepath: &str) -> Result<File, Box<dyn std::error::Error>> {
    if std::fs::canonicalize(Path::new(output_filepath)).unwrap_or_default() == std::fs::canonicalize(Path::new(input_filepath)).unwrap_or_default() {
        Err(KuuubeditError::InputOutputMatch)?
    }
    return Ok(File::create(output_filepath)?)
}

pub struct Commands {
    pub operation: Operation,
    pub find_regex: onig::Regex,
    pub replace: String,
    pub output_file: Option<File>,
    pub destructive: bool,
    pub view_start: u64,
    pub view_length: usize
}

impl Default for Commands {
    fn default() -> Self {
        Self {
            operation: Default::default(),
            find_regex: onig::Regex::new("").unwrap(),
            replace: Default::default(),
            output_file: Default::default(),
            destructive: Default::default(),
            view_start: Default::default(),
            view_length: Default::default()
        }
    }
}
