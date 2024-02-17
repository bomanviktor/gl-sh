use crate::commands::{
    cat, cd, clear, cp, date, echo, help, ls, mkdir, mv, pwd, rm, touch, uname, who_am_i,
};

fn execute(input: String) -> bool {
    let split_input = input.split_ascii_whitespace().collect::<Vec<&str>>();
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
        "exit" => return false,
        "help" => help(),
        "ls" => ls(args),
        "mkdir" => mkdir(args),
        "mv" => mv(args),
        "rm" => rm(args),
        "touch" => touch(args),
        "uname" => println!("{}", uname(&args)),
        "pwd" => pwd(),
        "whoami" => println!("{}", who_am_i()),
        _ => println!("Could not read command: {command}. Type 'help' to list available commands"),
    }
    true
}

pub fn execute_commands(input: &str) -> bool {
    let commands = input.split("&&").collect::<Vec<&str>>();

    for command in commands {
        if !execute(command.trim().to_string()) {
            return false;
        }
    }
    true
}
