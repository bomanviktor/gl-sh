use crate::commands::{uname, who_am_i};
use chrono::{Local, Timelike};
use std::env;
use termion::{color, style};
pub fn custom_prompt() -> String {
    let user = who_am_i();
    let host_name = uname("-n");

    let current_time = Local::now();
    let (hour, minute, second) = (
        current_time.hour(),
        current_time.minute(),
        current_time.second(),
    );
    let time = format!("{:02}:{:02}:{:02}", hour, minute, second);

    let current_dir = env::current_dir()
        .map(|dir| dir.display().to_string())
        .unwrap_or_else(|_| "".to_string());

    let path = current_dir.splitn(4, '/').skip(3).collect::<String>();
    format!(
        "{}{} {user}@{host_name} {}{} {time} {}{} ~/{path} {}{}{} $ ",
        // user + hostname
        color::Bg(color::Rgb(40, 40, 40)),
        color::Fg(color::Green),
        // time
        color::Bg(color::Rgb(60, 60, 60)),
        color::Fg(color::Yellow),
        // path
        color::Bg(color::Rgb(60, 60, 100)),
        color::Fg(color::LightCyan),
        // reset
        color::Bg(color::Reset),
        color::Fg(color::Reset),
        style::Reset
    )
}
