use crate::commands::{
    cat, cd, clear, cp, date, echo, help, ls, mkdir, mv, pwd, rm, touch, uname, who_am_i,
};
use crate::helpers::error::custom_error;
use crate::helpers::execute::ExecuteOption::*;
use crate::helpers::{parse_args, parse_flags, parse_input, redirect};

pub enum ExecuteOption {
    Out(String),
    Exit,
    Empty,
}

fn execute(input: String) -> ExecuteOption {
    let (command, input) = match parse_input(input) {
        Some(v) => v,
        None => return Empty,
    };
    let flags = parse_flags(&input);
    let args = parse_args(&input);
    match command.as_str() {
        "cat" => cat(args),
        "cd" => cd(args),
        "cp" => cp(args),
        "clear" => clear(),
        "date" => date(),
        "echo" => echo(args),
        "exit" => Exit,
        "help" => help(),
        "ls" => ls(flags, args),
        "mkdir" => mkdir(args),
        "mv" => mv(args),
        "rm" => rm(flags, args),
        "touch" => touch(args),
        "uname" => uname(flags),
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
