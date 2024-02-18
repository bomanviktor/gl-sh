use crate::commands::{pwd, uname, who_am_i};
use crate::helpers::execute::ExecuteOption::Out;
use chrono::{Local, Timelike};
use dirs::home_dir;
use termion::{color, style};

pub fn custom_prompt() -> String {
    let user = match who_am_i() {
        Out(v) => v,
        _ => "".to_string(),
    };
    let host_name = match uname("-n") {
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
    format!(
        "{}{} {user}@{host_name} {}{} {time} {}{} {path} {}{}{} $ ",
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
