use onig::Regex;

fn main() {
    let regex_input = read_line();
    let re = Regex::new(&regex_input).unwrap(); //(?<=(\"|\"1))test(?=\")
    let match_test = re.captures_iter("craietciaestnaciesrn\"test\"aicerntaiecnteain\"1test\"");
    for captures in match_test {
        println!("{}", captures.at(0).unwrap());
    }
}

fn read_line() -> String {
    print!(":");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let mut buffer = String::default();
    std::io::stdin().read_line(&mut buffer).unwrap_or_default();
    let len = buffer.trim_end_matches(&['\r', '\n'][..]).len();
    buffer.truncate(len);
    return buffer;
}