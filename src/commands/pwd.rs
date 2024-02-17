use crate::helpers::error::custom_error;
use std::env;

pub fn pwd() {
    if let Ok(current_dir) = env::current_dir() {
        println!("{}", current_dir.display());
    } else {
        custom_error("Error", "Unable to determine current directory.");
    }
}
