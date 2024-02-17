use std::env;

pub fn pwd() {
    if let Ok(current_dir) = env::current_dir() {
        println!("{}", current_dir.display());
    } else {
        println!("Error: Unable to determine the current directory.");
    }
}
