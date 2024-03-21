use crate::operations::Operation;

pub fn get_commands() -> Commands {
    let input_string = read_line();
    let mut commands: Commands = Default::default();
    let input_split: Vec<&str> = input_string.split(" ").collect();
    match input_split.get(0).unwrap() {
        &"f" => commands.operation = Operation::Find,
        &"r" => commands.operation = Operation::Replace,
        &"rw" => commands.operation = Operation::ReplaceWrite,
        &"w" => commands.operation = Operation::Write,
        &"o" => commands.operation = Operation::Output,
        &"u" => commands.operation = Operation::Undo,
        &"q" => commands.operation = Operation::Quit,
        _ => commands.operation = Operation::None
    }

    match (input_split.len(), &commands.operation) {
        (3, Operation::Find) => {
            commands.find = input_split.get(1).unwrap_or(&"").to_string();
            commands.output_file = input_split.get(2).unwrap_or(&"").to_string();
        },
        (3, Operation::Replace) => {
            commands.find = input_split.get(1).unwrap_or(&"").to_string();
            commands.replace = input_split.get(2).unwrap_or(&"").to_string();
            commands.destructive = true;
        },
        (4, Operation::ReplaceWrite) => {
            commands.find = input_split.get(1).unwrap_or(&"").to_string();
            commands.replace = input_split.get(2).unwrap_or(&"").to_string();
            commands.output_file = input_split.get(3).unwrap_or(&"").to_string();
            commands.destructive = true;
        },
        (2, Operation::Write) => {
            commands.output_file = input_split.get(1).unwrap_or(&"").to_string();
        },
        (1, Operation::Output | Operation::Undo | Operation::Quit) => {},
        (_, Operation::None) => {},
        (_, _) => {commands.operation = Operation::None; println!("Incorrect number of command params")}

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
