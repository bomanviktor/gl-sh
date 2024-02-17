use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;
use std::fs;

pub fn cp(args: String) {
    let args = args.split_ascii_whitespace().collect::<Vec<&str>>();
    if args.len() != 2 {
        eprintln!("Usage: cp <source> <destination>");
        return;
    }
    let arg1 = args[0];
    let arg2 = args[1];

    let mut path = get_absolute_path();
    let source = format!("{path}/{arg1}");

    if arg2 == "." {
        let destination = format!("{path}/{}", arg1.rsplit_once('/').unwrap().1);
        if let Err(e) = fs::copy(source, destination) {
            command_error("cp", e, arg2.to_string());
        }
        return;
    }

    if arg2.starts_with("..") {
        path = traverse_back(arg2);
    }

    if arg2.starts_with('~') {
        path = traverse_home(arg2);
    }

    let destination = format!("{path}/{arg1}");
    if let Err(e) = fs::copy(source, destination) {
        let args = format!("{arg1} {arg2}");
        command_error("cp", e, args);
    }
}
