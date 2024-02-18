use crate::commands::{get_absolute_path, traverse_back, traverse_home};
use crate::helpers::command_error;

use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Out;
use std::fs::File;
use std::io::Read;
use termion::color;

pub fn cat(args: Vec<&str>) -> ExecuteOption {
    let mut output = String::new();
    let mut syntax_highlighting = false;
    for arg in &args {
        if arg.ends_with(&".gsh") {
            syntax_highlighting = true;
        }

        let mut path = format!("{}/{arg}", get_absolute_path());
        if arg.starts_with("..") {
            path = traverse_back(arg);
        }

        if arg.starts_with('~') || args.is_empty() {
            path = traverse_home(arg);
        }

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                command_error("cat", e, arg);
                continue;
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            command_error("cat", e, arg);
            continue;
        }
        if syntax_highlighting {
            output.push_str(&highlighting(&contents));
        } else {
            output.push_str(&contents);
        }
    }

    Out(output.trim_end_matches('\n').to_string())
}

fn highlighting(output: &str) -> String {
    let commands_to_highlight = [
        "else", "for", "in", "ls", "cat", "cd", "cp", "clear", "date", "echo", "exit", "help",
        "gsh", "mkdir", "mv", "rm", "touch", "uname", "pwd", "ver", "whoami", "$",
    ];

    let orange_color = color::Fg(color::Rgb(200, 165, 0));
    let reset_color = color::Fg(color::Reset);

    let mut highlighted_output = output.to_string();

    for command in commands_to_highlight.iter() {
        let replacement = &format!("{}{}{}", orange_color, command, reset_color);
        highlighted_output = highlighted_output.replace(command, replacement);
    }

    highlighted_output
}
