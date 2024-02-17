use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Out;

pub fn help() -> ExecuteOption {
    let mut output = String::from("Available commands:");
    for command in [
        "echo", "ls", "clear", "cd", "mv", "cp", "mkdir", "rm", "exit", "pwd", "cat", "date",
        "uname", "whoami",
    ] {
        output.push_str(&format!("\n -{command}"));
    }
    Out(output)
}
