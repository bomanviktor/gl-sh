use std::io::Error;
use termion::{color, style};

pub fn command_error(command: &str, e: Error, args: String) {
    let error = e
        .to_string()
        .to_ascii_lowercase()
        .split_once('(')
        .unwrap()
        .0
        .trim()
        .to_string();

    eprint!("{}", color::Fg(color::Red));
    eprint!("{command}: {error}: ");
    eprint!("{}", color::Fg(color::Cyan));
    eprint!("{}", style::Underline);
    eprintln!("{args}");
    eprint!("{}", color::Fg(color::Reset));
    eprint!("{}", style::Reset);
}
