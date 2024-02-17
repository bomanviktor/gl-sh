use std::fs::OpenOptions;
use std::io::Write;
pub fn add_to_history(input: &str) {
    let contents = input.to_string();
    // Open the file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("./src/history.txt")
        .unwrap();
    // Write the content to the file
    writeln!(file, "{}", contents).unwrap();
}
