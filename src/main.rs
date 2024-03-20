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

    let regex_input = read_line();
    let re = match Regex::new(&regex_input) { //(?<=(\"|\"1))test(?=\")
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

fn read_line() -> String {
    print!(":");
    std::io::Write::flush(&mut std::io::stdout()).unwrap_or_default();
    let mut buffer = String::default();
    std::io::stdin().read_line(&mut buffer).unwrap_or_default();
    let len = buffer.trim_end_matches(&['\r', '\n'][..]).len();
    buffer.truncate(len);
    return buffer;
}