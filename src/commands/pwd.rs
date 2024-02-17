use crate::helpers::error::custom_error;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::{Empty, Out};
use std::env;

pub fn pwd() -> ExecuteOption {
    if let Ok(current_dir) = env::current_dir() {
        Out(format!("{}", current_dir.display()))
    } else {
        custom_error("Error", "Unable to determine current directory.");
        Empty
    }
}
