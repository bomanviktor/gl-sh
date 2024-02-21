use crate::commands::{get_absolute_path, traverse_home};
use crate::helpers::command_error;
use crate::helpers::error::custom_error;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::{Empty, Out};
use std::env;

pub fn cd(args: Vec<&str>, last_location: bool) -> ExecuteOption {
    if args.len() > 1 {
        custom_error("cd", &format!("string not in pwd: {}", args.join(" ")));
        return Empty;
    }

    let destination = args[0];
    if last_location {
        env::set_current_dir(destination).unwrap_or_else(|e| {
            command_error("cd", e, destination);
        });
        if destination == "///" {
            return Out("/".to_string());
        } else {
            return Out(destination.to_string());
        }
    }

    if destination == "/" {
        env::set_current_dir("/").unwrap_or_else(|e| {
            command_error("cd", e, destination);
        });
    }

    let absolute_path = get_absolute_path();
    let mut path = format!("{absolute_path}/{destination}");
    if destination.starts_with('~') || destination.is_empty() {
        path = traverse_home(destination);
    }
    env::set_current_dir(&path).unwrap_or_else(|e| {
        command_error("cd", e, destination);
    });

    Out(path)
}
