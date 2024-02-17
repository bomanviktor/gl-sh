use std::env;

pub fn who_am_i() -> String {
    let username = match env::consts::OS {
        "linux" | "macos" => env::var("USER"),
        "windows" => env::var("USERNAME"),
        _ => return "".to_string(),
    };
    username.unwrap()
}
