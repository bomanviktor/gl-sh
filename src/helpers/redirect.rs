use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use std::fs;

pub fn redirect(path: String, output: String) {
    let mut new_path = format!("{}/{}", get_absolute_path(), path);
    if path.starts_with("..") {
        new_path = format!("{}/{path}", traverse_back(&path));
    }

    if path.starts_with('~') {
        new_path = format!("{}/{path}", traverse_home(&path));
    }
    fs::write(new_path, output).unwrap()
}
