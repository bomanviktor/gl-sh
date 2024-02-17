use crate::commands::{get_absolute_path, traverse_back};
use std::fs;

pub fn mkdir(args: String) {
    let mut path = get_absolute_path();
    if args.contains("../") {
        path = traverse_back(&args);
    }
    if let Err(e) = fs::create_dir_all(path) {
        eprintln!("mkdir: {e}");
    }
}
