use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;
use std::fs;

pub fn rm(args: String) {
    let mut input = args;
    let mut recursive = false;
    if input.contains("-r") {
        recursive = true;
        input = input.replace("-r", "").trim().to_string();
    }
    let args = input.split_ascii_whitespace().collect::<Vec<&str>>();
    for arg in args.iter().map(|a| a.trim()) {
        let mut path = format!("{}/{arg}", get_absolute_path());
        if arg.starts_with("../") {
            path = traverse_back(arg);
        }

        if arg.starts_with('~') {
            path = traverse_home(arg);
        }

        if fs::metadata(&path).unwrap().is_file() {
            if let Err(e) = fs::remove_file(&path) {
                command_error("rm", e, arg);
            }
            continue;
        }

        if recursive {
            if let Err(e) = fs::remove_dir_all(&path) {
                command_error("rm", e, arg);
            }
        } else if let Err(e) = fs::remove_dir(&path) {
            command_error("rm", e, arg);
        }
    }
}
