use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Empty;
use std::fs;

pub fn rm(flags: Vec<&str>, args: Vec<&str>) -> ExecuteOption {
    let mut recursive = false;
    if flags.contains(&"-r") {
        recursive = true;
    }
    for arg in args.iter().map(|a| a.trim()) {
        let mut path = format!("{}/{arg}", get_absolute_path());
        if arg.starts_with("../") {
            path = traverse_back(arg);
        }

        if arg.starts_with('~') {
            path = traverse_home(arg);
        }

        match fs::metadata(&path) {
            Ok(md) => {
                if md.is_file() {
                    if let Err(e) = fs::remove_file(&path) {
                        command_error("rm", e, arg);
                    }
                    continue;
                }
            }
            Err(e) => {
                command_error("rm", e, arg);
                continue;
            }
        }

        if recursive {
            if let Err(e) = fs::remove_dir_all(&path) {
                command_error("rm", e, arg);
            }
        } else if let Err(e) = fs::remove_dir(&path) {
            command_error("rm", e, arg);
        }
    }
    Empty
}
