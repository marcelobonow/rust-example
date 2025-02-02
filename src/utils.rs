use std::io::Write;

pub fn parse_string(string: &str) -> String {
    let mut parsed = String::new();
    for char in string.chars() {
        if char == ';' || char == '\n' || char == '/' {
            parsed.push('/');
        }
        parsed.push(char);
    }
    return parsed;
}

pub fn flush() {
    return std::io::stdout().flush().unwrap();
}

pub fn read_line(print_text: &str, input: &mut String) -> Result<usize, std::io::Error> {
    print!("{}", print_text);
    flush();

    let result = std::io::stdin().read_line(input);
    return result;
}
