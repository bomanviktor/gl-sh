use crate::commands::{get_absolute_path, traverse_home};
use crate::helpers::command_error;
use crate::helpers::error::usage_error;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Empty;
use std::fs;

pub fn cp(args: Vec<&str>) -> ExecuteOption {
    if args.len() != 2 {
        usage_error("cp", "<source> <destination>");
        return Empty;
    }
    let arg1 = args[0];
    let arg2 = args[1];

    let mut path = get_absolute_path();
    let source = format!("{path}/{arg1}");

    if arg2 == "." {
        let destination = format!("{path}/{}", arg1.rsplit_once('/').unwrap().1);
        if let Err(e) = fs::copy(source, destination) {
            command_error("cp", e, arg2);
        }
        return Empty;
    }

    if arg2.starts_with('~') {
        path = traverse_home(arg2);
    }

    let destination = format!("{path}/{arg2}");
    if let Err(e) = fs::copy(source, destination) {
        let args = format!("{arg1} {arg2}");
        command_error("cp", e, &args);
    }
    Empty
}
