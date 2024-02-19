pub fn uname(flags: Vec<&str>) -> ExecuteOption {
    if flags.is_empty() {
        return Out(system());
    }

    let mut output = Vec::new();
    if flags.contains(&"-n") {
        output.push(name());
    }

    if flags.contains(&"-s") {
        output.push(system());
    }

    Out(output.join(" "))
}

fn system() -> String {
    match env::consts::OS {
        "linux" => "Linux",
        "macos" => "macOS",
        "windows" => "Windows",
        "android" => "Android",
        "ios" => "iOS",
        "freebsd" => "FreeBSD",
        // Add more OS mappings as needed
        _ => "Unknown",
    }
    .to_string()
}

extern crate libc;

use crate::helpers::error::custom_error;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Out;
use libc::{c_char, gethostname};
use std::env;
use std::ffi::CStr;

fn name() -> String {
    const HOSTNAME_BUFFER_SIZE: usize = 256;
    let mut buffer: [c_char; HOSTNAME_BUFFER_SIZE] = [0; HOSTNAME_BUFFER_SIZE];

    if unsafe { gethostname(buffer.as_mut_ptr(), HOSTNAME_BUFFER_SIZE as libc::size_t) } == 0 {
        let hostname = unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy() };
        return hostname.replace(".lan", "").to_string();
    } else {
        custom_error("Error:", "Unable to determine the hostname.");
    }
    "".to_string()
}
