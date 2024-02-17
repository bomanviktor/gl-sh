use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;

use std::fs::File;
use std::io::Read;

pub fn cat(args: String) {
    let mut path = format!("{}/{args}", get_absolute_path());
    if args.starts_with("..") {
        path = traverse_back(&args);
    }

    if args.starts_with('~') || args.is_empty() {
        path = traverse_home(&args);
    }

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            command_error("cat", e, args);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        command_error("cat", e, args);
        return;
    }
    print!("{}", contents);
}
