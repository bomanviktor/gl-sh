use std::io::Error;
use termion::{color, style};

pub fn command_error(command: &str, e: Error, args: &str) {
    let error = e.to_string().to_ascii_lowercase().trim().to_string();

    eprint!("{}", color::Fg(color::Red));
    eprint!("{command}: {error}: ");
    eprint!("{}", color::Fg(color::Cyan));
    eprint!("{}", style::Underline);
    eprintln!("{args}");
    eprint!("{}", color::Fg(color::Reset));
    eprint!("{}", style::Reset);
}

pub fn usage_error(command: &str, instructions: &str) {
    eprint!("Usage: ");
    eprint!("{}", color::Fg(color::Cyan));
    eprintln!("{command} {instructions}");
    eprint!("{}", color::Fg(color::Reset))
}

pub fn custom_error(error_type: &str, desc: &str) {
    eprint!("{}", color::Fg(color::Red));
    eprint!("{error_type}: ");
    eprint!("{}", color::Fg(color::Cyan));
    eprintln!("{desc}");
    eprint!("{}", color::Fg(color::Reset))
}
