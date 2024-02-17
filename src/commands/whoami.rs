use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::{Empty, Out};
use std::env;

pub fn who_am_i() -> ExecuteOption {
    let username = match env::consts::OS {
        "linux" | "macos" => env::var("USER"),
        "windows" => env::var("USERNAME"),
        _ => return Empty,
    };
    Out(username.unwrap())
}
