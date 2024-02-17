use crate::commands::traverse_home;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
pub const HISTORY_PATH: &str = ".gl-sh_history";

pub fn init_history() {
    if let Some(mut path) = dirs::home_dir() {
        path.push(HISTORY_PATH);
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .unwrap();
        // Rest of the code remains the same
    } else {
        eprintln!("Unable to determine home directory");
    }
}

pub fn add_to_history(input: &str) {
    let contents = input.to_string();
    let file_path = traverse_home(HISTORY_PATH);

    // Open the file in append mode
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .unwrap();

    // Write the content to the file
    writeln!(file, "{}", contents).unwrap();

    // Check if the file has more than 1k lines
    if line_count(&file_path).unwrap_or(0) > 1000 {
        // Remove the first line
        remove_first_line(&file_path).unwrap();
    }
}

fn line_count(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

fn remove_first_line(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().skip(1).map(|line| line.unwrap()).collect();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
