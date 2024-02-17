use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Empty;
use std::env;

pub fn cd(args: String) -> ExecuteOption {
    let absolute_path = get_absolute_path();
    let mut path = format!("{absolute_path}/{args}");
    if args.starts_with("..") {
        path = traverse_back(&args);
    }

    if args.starts_with('~') || args.is_empty() {
        path = traverse_home(&args);
    }
    env::set_current_dir(path).unwrap_or_else(|e| {
        command_error("cd", e, &args);
    });

    Empty
}
