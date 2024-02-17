use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;
use std::fs;
use crate::helpers::error::usage_error;

pub fn mv(args: String) {
    let args = args.split_ascii_whitespace().collect::<Vec<&str>>();
    if args.len() != 2 {
        usage_error("mv", "<source> <destination>");
        return;
    }
    let arg1 = args[0];
    let arg2 = args[1];

    let mut path = get_absolute_path();
    let source = format!("{path}/{arg1}");

    if arg2 == "." {
        let destination = format!("{path}/{}", arg1.rsplit_once('/').unwrap().1);
        if let Err(e) = fs::rename(source, destination) {
            command_error("mv", e, arg2);
        }
        return;
    }

    if arg2.contains("..") {
        path = traverse_back(arg2);
    }

    if arg2.starts_with('~') {
        path = traverse_home(arg2);
    }

    let destination = format!("{path}/{arg1}");
    if let Err(e) = fs::rename(source, destination) {
        let args = format!("{arg1} {arg2}");
        command_error("mv", e, &args);
    }
}
