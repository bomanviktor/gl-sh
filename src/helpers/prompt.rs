use crate::commands::{pwd, uname, who_am_i};
use crate::helpers::execute::ExecuteOption::Out;
use chrono::{Local, Timelike};
use dirs::home_dir;
use std::fs;
use termion::{color, style};

fn git_path() -> Option<String> {
    let Out(mut full_path) = pwd() else {
        panic!("RIP!")
    };
    while let Some((parent, current)) = full_path.rsplit_once('/') {
        let git_path = format!("{parent}/{current}/.git/HEAD");
        if let Ok(file) = fs::read(git_path) {
            let file_ref = String::from_utf8_lossy(&file).to_string();
            let ref_path = file_ref.split_ascii_whitespace().collect::<Vec<&str>>()[1];
            let head = ref_path.replace("refs/heads/", "");
            return Some(head.to_string());
        }
        full_path = parent.to_string().clone();
    }
    None
}
pub fn custom_prompt() -> String {
    let user = match who_am_i() {
        Out(v) => v,
        _ => "".to_string(),
    };
    let host_name = match uname(vec!["-n"]) {
        Out(v) => v,
        _ => "".to_string(),
    };

    let current_time = Local::now();
    let (hour, minute, second) = (
        current_time.hour(),
        current_time.minute(),
        current_time.second(),
    );
    let time = format!("{:02}:{:02}:{:02}", hour, minute, second);
    let Out(mut path) = pwd() else { panic!() };
    let home_dir = home_dir().unwrap().to_string_lossy().to_string();
    path = path.replace(&home_dir, "~");
    if path.eq("~") {
        path.push('/');
    }

    let git = match git_path() {
        Some(path) => {
            format!(
                "{}{} {path} {}{}",
                color::Bg(color::LightGreen),
                color::Fg(color::Black),
                color::Bg(color::Reset),
                color::Fg(color::LightGreen)
            )
        }
        None => "".to_string(),
    };

    let fs_path = if git.is_empty() {
        format!(
            " {}{} {path} {}{}",
            color::Bg(color::Rgb(60, 60, 100)),
            color::Fg(color::LightCyan),
            color::Bg(color::Reset),
            color::Fg(color::Rgb(60, 60, 100))
        )
    } else {
        format!(
            " {}{} {path} {}{}",
            color::Bg(color::Rgb(60, 60, 100)),
            color::Fg(color::LightCyan),
            color::Bg(color::LightGreen),
            color::Fg(color::Rgb(60, 60, 100))
        )
    };

    let reset = format!(
        "{}{}{}",
        color::Bg(color::Reset),
        color::Fg(color::Reset),
        style::Reset
    );

    format!(
        "{}{} {user}@{host_name} {}{} {time}{fs_path}{git}{reset} ",
        // user + hostname
        color::Bg(color::Rgb(40, 40, 40)),
        color::Fg(color::Green),
        // time
        color::Bg(color::Rgb(60, 60, 60)),
        color::Fg(color::Yellow),
    )
}
