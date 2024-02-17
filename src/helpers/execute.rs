use crate::commands::{
    cat, cd, clear, cp, date, echo, help, ls, mkdir, mv, pwd, rm, touch, uname, who_am_i,
};
use crate::helpers::error::custom_error;
use crate::helpers::execute::ExecuteOption::*;
use crate::helpers::redirect;

pub enum ExecuteOption {
    Out(String),
    Exit,
    Empty,
}
fn execute(input: String) -> ExecuteOption {
    let split_input = input.split_ascii_whitespace().collect::<Vec<&str>>();
    if split_input.is_empty() {
        return Empty;
    }
    let command = split_input[0];
    let args = if split_input.len() > 1 {
        split_input[1..].join(" ")
    } else {
        "".to_string()
    };
    match command {
        "cat" => cat(args),
        "cd" => cd(args),
        "cp" => cp(args),
        "clear" => clear(),
        "date" => date(),
        "echo" => echo(args),
        "exit" => Exit,
        "help" => help(),
        "ls" => ls(args),
        "mkdir" => mkdir(args),
        "mv" => mv(args),
        "rm" => rm(args),
        "touch" => touch(args),
        "uname" => uname(&args),
        "pwd" => pwd(),
        "whoami" => who_am_i(),
        _ => {
            custom_error(
                "Could not read command",
                "Type 'help' to list available commands.",
            );
            Empty
        }
    }
}

pub fn execute_commands(input: &str) -> bool {
    let commands = input.split("&&").collect::<Vec<&str>>();
    for command in commands {
        let mut input;
        let mut output = String::new();
        let mut redirection_path = String::new();
        let pipes = command.split('|').collect::<Vec<&str>>();

        for pipe in pipes {
            if let Some((before, after)) = pipe.split_once('>') {
                input = format!("{before} {output}").trim().to_string();
                redirection_path = after.to_string();
            } else {
                input = format!("{pipe} {output}").trim().to_string();
            }
            match execute(input) {
                Out(v) => output = v,
                Empty => {
                    output.clear();
                    continue;
                }
                Exit => return false,
            };
        }
        // All non-error output will display here.
        if !output.is_empty() && redirection_path.is_empty() {
            println!("{output}");
        } else if !output.is_empty() && !redirection_path.is_empty() {
            redirect(redirection_path, output);
        }
    }
    true
}
