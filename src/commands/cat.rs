use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;

use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Out;
use std::fs::File;
use std::io::Read;

pub fn cat(args: Vec<&str>) -> ExecuteOption {
    let mut output = String::new();
    for arg in &args {
        let mut path = format!("{}/{arg}", get_absolute_path());
        if arg.starts_with("..") {
            path = traverse_back(arg);
        }

        if arg.starts_with('~') || args.is_empty() {
            path = traverse_home(arg);
        }

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                command_error("cat", e, arg);
                continue;
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            command_error("cat", e, arg);
            continue;
        }
        output.push_str(&contents);
    }
    Out(output.trim_end_matches('\n').to_string())
}
