pub fn uname(input: &str) -> String {
    let mut input = input.to_string();
    if input.contains("uname") {
        input = input.replace("uname", "").trim().to_string();
    }

    if input.is_empty() {
        return system();
    }

    let mut output = Vec::new();

    if input.contains("-n") {
        output.push(name());
    }

    if input.contains("-s") {
        output.push(system());
    }

    output.join(" ")
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

use libc::{c_char, gethostname};
use std::env;
use std::ffi::CStr;

fn name() -> String {
    const HOSTNAME_BUFFER_SIZE: usize = 256;
    let mut buffer: [c_char; HOSTNAME_BUFFER_SIZE] = [0; HOSTNAME_BUFFER_SIZE];

    if unsafe { gethostname(buffer.as_mut_ptr(), HOSTNAME_BUFFER_SIZE as libc::size_t) } == 0 {
        let hostname = unsafe { CStr::from_ptr(buffer.as_ptr()).to_string_lossy() };
        return hostname.rsplit_once('.').unwrap().0.to_string();
    } else {
        println!("Unable to determine the hostname.");
    }
    "".to_string()
}
