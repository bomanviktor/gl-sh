use crate::commands::{get_absolute_path, traverse_back};
use std::fs;
use crate::helpers::command_error;

pub fn mkdir(args: String) {
    let args = args.split_ascii_whitespace().collect::<Vec<&str>>();
    let mut path = get_absolute_path();
    for arg in &args {
        if arg.starts_with("../") {
            path = traverse_back(arg);
        }

        if arg.starts_with("~") {
            path = traverse_back(arg);
        }
        if let Err(e) = fs::create_dir_all(&path) {
            command_error("mkdir", e, arg);
        }

    }

}
