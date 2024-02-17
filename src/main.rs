use gritlab_shell::commands::traverse_home;
use gritlab_shell::helpers::history::{init_history, HISTORY_PATH};
use gritlab_shell::helpers::{custom_prompt, execute_commands};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;

fn main() {
    let mut rl = Editor::<()>::new();
    init_history();
    let file_path = format!("{}/{}", traverse_home(""), HISTORY_PATH);
    if rl.load_history(&file_path).is_err() {
        println!("Failed to load history. Exiting gl-sh...");
        return;
    }

    let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
    env::set_current_dir(current_dir).expect("Could not start the shell.");

    loop {
        let readline = rl.readline(custom_prompt().as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if !execute_commands(line.trim()) {
                    if let Err(err) = rl.save_history(&file_path) {
                        println!("Error saving history: {:?}", err);
                    }
                    return;
                };
            }
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => continue,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    if let Err(err) = rl.save_history(&file_path) {
        println!("Error saving history: {:?}", err);
    }
}
