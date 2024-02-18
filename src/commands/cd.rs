use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;
use crate::helpers::error::custom_error;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Empty;
use std::env;

pub fn cd(args: Vec<&str>) -> ExecuteOption {
    if args.len() > 1 {
        custom_error("cd", &format!("string not in pwd: {}", args.join(" ")));
        return Empty;
    }
    let destination = args[0];
    let absolute_path = get_absolute_path();
    let mut path = format!("{absolute_path}/{destination}");
    if destination.starts_with("..") {
        path = traverse_back(args[0]);
    }

    if destination.starts_with('~') || destination.is_empty() {
        path = traverse_home(destination);
    }
    env::set_current_dir(path).unwrap_or_else(|e| {
        command_error("cd", e, destination);
    });

    Empty
}
