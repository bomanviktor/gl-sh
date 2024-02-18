use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Empty;
use std::fs;

pub fn touch(args: Vec<&str>) -> ExecuteOption {
    for arg in args.iter().map(|a| a.trim()) {
        let mut path = format!("{}/{arg}", get_absolute_path());
        if arg.starts_with("..") {
            path = traverse_back(arg);
        }
        if arg.starts_with('~') {
            path = traverse_home(arg);
        }
        if let Err(e) = fs::write(path, "") {
            command_error("touch", e, arg);
        }
    }
    Empty
}
