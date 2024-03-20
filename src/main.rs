use onig::Regex;

fn main() {
    let regex_input = strip_newlines(read_line());
    let re = Regex::new(&regex_input).unwrap(); //(?<=(\"|\"1))test(?=\")
    let match_test = re.captures_iter("craietciaestnaciesrn\"test\"aicerntaiecnteain\"1test\"");
    for captures in match_test {
        println!("{}", captures.at(0).unwrap());
    }
}

fn read_line() -> String {
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let mut buffer = String::default();
    std::io::stdin().read_line(&mut buffer).unwrap_or_default();
    return buffer;
}

fn strip_newlines(input_string: String) -> String {
    let mut_input_string: &mut String = &mut input_string.clone();
    let len = mut_input_string.trim_end_matches(&['\r', '\n'][..]).len();
    mut_input_string.truncate(len);
    return mut_input_string.to_owned();
}
