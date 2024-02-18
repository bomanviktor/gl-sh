pub fn parse_input(input: String) -> Option<(String, String)> {
    let split_input = input.split_ascii_whitespace().collect::<Vec<&str>>();
    if split_input.is_empty() {
        return None;
    }
    let command = split_input[0].to_string();
    let input = if split_input.len() > 1 {
        split_input[1..].join(" ")
    } else {
        String::new()
    };
    Some((command, input))
}

pub fn parse_flags(input: &str) -> Vec<&str> {
    input
        .split_ascii_whitespace()
        .take_while(|a| a.starts_with('-'))
        .collect::<Vec<&str>>()
}

pub fn parse_args(input: &str) -> Vec<&str> {
    let mut args = input
        .split_ascii_whitespace()
        .skip_while(|a| a.starts_with('-'))
        .collect::<Vec<&str>>();

    if args.is_empty() {
        args.push("");
    }
    args
}
