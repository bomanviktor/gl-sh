pub fn help() {
    println!("Available commands:");
    for command in [
        "echo", "ls", "clear", "cd", "mv", "cp", "mkdir", "rm", "exit", "pwd", "cat", "date",
        "uname", "whoami",
    ] {
        println!("- {command}");
    }
}
