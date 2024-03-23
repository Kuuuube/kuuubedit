use crate::operations::Operation;

pub fn get_commands() -> Option<Commands> {
    let input_string = read_line();
    let mut commands: Commands = Default::default();
    let input_split: Vec<String> = parse_command(&input_string);
    match input_split.get(0).unwrap().as_str() {
        "f" => commands.operation = Operation::Find,
        "r" => commands.operation = Operation::Replace,
        "rw" => commands.operation = Operation::ReplaceWrite,
        "w" => commands.operation = Operation::Write,
        "o" => commands.operation = Operation::Output,
        "u" => commands.operation = Operation::Undo,
        "q" => commands.operation = Operation::Quit,
        _ => commands.operation = Operation::None
    }

    match &commands.operation {
        Operation::Find => {
            commands.find = input_split.get(1)?.to_string();
            commands.output_file = input_split.get(2)?.to_string();
        },
        Operation::Replace => {
            commands.find = input_split.get(1)?.to_string();
            commands.replace = input_split.get(2)?.to_string();
            commands.destructive = true;
        },
        Operation::ReplaceWrite => {
            commands.find = input_split.get(1)?.to_string();
            commands.replace = input_split.get(2)?.to_string();
            commands.output_file = input_split.get(3)?.to_string();
            commands.destructive = true;
        },
        Operation::Write => {
            commands.output_file = input_split.get(1)?.to_string();
        },
        Operation::Output | Operation::Undo | Operation::Quit => {},
        Operation::None => return None
    };
    return Some(commands);
}

fn parse_command(command_string: &str) -> Vec<String> {
    let mut chars_vec: Vec<char> = Default::default();
    let command_chars: Vec<char> = command_string.chars().collect();
    let mut command_split: Vec<String> = Default::default();
    let mut inside_quotes: bool = false;
    let mut i: usize = 0;
    while i < command_chars.len() {
        let current_char = command_chars.get(i).unwrap_or(&char::default()).to_owned();
        let next_char = command_chars.get(i + 1).unwrap_or(&char::default()).to_owned();
        match (current_char, next_char, inside_quotes) {
            (' ', _, false) => {
                command_split.push(chars_vec.into_iter().collect());
                chars_vec = Default::default();
            },
            ('\\', '"', true) => {
                chars_vec.push('"');
                i += 1;
            },
            ('"', _, false) => {
                inside_quotes = true;
            },
            ('"', _, true) => {
                inside_quotes = false;
            },
            _ => { chars_vec.push(current_char) }
        }
        i += 1;
        if i == command_chars.len() {
            command_split.push(chars_vec.into_iter().collect());
            chars_vec = Default::default();
        }
    }
    return command_split;
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
pub struct Commands {
    pub operation: Operation,
    pub find: String,
    pub replace: String,
    pub output_file: String,
    pub destructive: bool
}
