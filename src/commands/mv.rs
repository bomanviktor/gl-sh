use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;
use crate::helpers::error::usage_error;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Empty;
use std::fs;

pub fn mv(args: Vec<&str>) -> ExecuteOption {
    if args.len() != 2 {
        usage_error("mv", "<source> <destination>");
        return Empty;
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
        return Empty;
    }

    if arg2.starts_with('~') {
        path = traverse_home(arg2).trim_end_matches('/').to_string();
    }

    if arg2.starts_with("..") {
        path = traverse_back(arg2).trim_end_matches('/').to_string();
    }

    let destination = format!("{path}/{arg2}");
    if let Ok(md) = fs::metadata(&destination) {
        if md.is_dir() {
            let new_destination = format!("{destination}/{arg1}");
            if let Err(e) = fs::rename(source, new_destination) {
                command_error("mv", e, arg1);
            }
            return Empty;
        }
    }
    if let Err(e) = fs::rename(source, destination) {
        command_error("mv", e, arg1);
    }

    Empty
}
