use crate::operations::Operation;

pub fn get_commands() -> Option<Commands> {
    let input_string = read_line();
    let mut commands: Commands = Default::default();
    let input_split: Vec<String> = parse_command(&input_string);
    match input_split.get(0).unwrap().as_str() {
        "f" => commands.operation = Operation::Find,
        "r" => commands.operation = Operation::Replace,
        "ra" => commands.operation = Operation::ReplaceActive,
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
            commands.replace = unescape(input_split.get(2)?);
            commands.output_file = input_split.get(3)?.to_string();
            commands.destructive = true;
        },
        Operation::ReplaceActive => {
            commands.find = input_split.get(1)?.to_string();
            commands.replace = unescape(input_split.get(2)?);
            commands.destructive = true;
            commands.no_buffer = true;
        },
        Operation::Write => {
            commands.output_file = input_split.get(1)?.to_string();
        },
        Operation::Output | Operation::Undo => {commands.no_buffer = true;},
        Operation::Quit => {},
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
            ('\\', '"', _) => {
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

#[derive(Default)]
pub struct Commands {
    pub operation: Operation,
    pub find: String,
    pub replace: String,
    pub output_file: String,
    pub destructive: bool,
    pub no_buffer: bool
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

//https://github.com/BurntSushi/ripgrep/blob/a2e6aec7a4d9382941932245e8854f0ae5703a5e/crates/cli/src/escape.rs
pub fn unescape(s: &str) -> String {
    use self::State::*;

    let mut bytes = vec![];
    let mut state = Literal;
    for c in s.chars() {
        match state {
            Escape => match c {
                '\\' => {
                    bytes.push(b'\\');
                    state = Literal;
                }
                'n' => {
                    bytes.push(b'\n');
                    state = Literal;
                }
                'r' => {
                    bytes.push(b'\r');
                    state = Literal;
                }
                't' => {
                    bytes.push(b'\t');
                    state = Literal;
                }
                'x' => {
                    state = HexFirst;
                }
                c => {
                    bytes.extend(format!(r"\{}", c).into_bytes());
                    state = Literal;
                }
            },
            HexFirst => match c {
                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                    state = HexSecond(c);
                }
                c => {
                    bytes.extend(format!(r"\x{}", c).into_bytes());
                    state = Literal;
                }
            },
            HexSecond(first) => match c {
                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                    let ordinal = format!("{}{}", first, c);
                    let byte = u8::from_str_radix(&ordinal, 16).unwrap();
                    bytes.push(byte);
                    state = Literal;
                }
                c => {
                    let original = format!(r"\x{}{}", first, c);
                    bytes.extend(original.into_bytes());
                    state = Literal;
                }
            },
            Literal => match c {
                '\\' => {
                    state = Escape;
                }
                c => {
                    bytes.extend(c.to_string().as_bytes());
                }
            },
        }
    }
    match state {
        Escape => bytes.push(b'\\'),
        HexFirst => bytes.extend(b"\\x"),
        HexSecond(c) => bytes.extend(format!("\\x{}", c).into_bytes()),
        Literal => {}
    }
    std::str::from_utf8(&bytes).unwrap().to_string()
}

enum State {
    /// The state after seeing a `\`.
    Escape,
    /// The state after seeing a `\x`.
    HexFirst,
    /// The state after seeing a `\x[0-9A-Fa-f]`.
    HexSecond(char),
    /// Default state.
    Literal,
}
