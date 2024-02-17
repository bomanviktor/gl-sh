use gritlab_shell::helpers::{custom_prompt, execute_commands};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;

fn main() {
    let mut rl = Editor::<()>::new();
    let file_path = "./src/history.txt";

    if rl.load_history(file_path).is_err() {
        println!("Failed to load history. Exiting gl-sh...");
        return;
    }

    env::set_current_dir(env::current_dir().unwrap()).expect("Could not start the shell.");

    loop {
        let readline = rl.readline(custom_prompt().as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if !execute_commands(line.trim()) {
                    if let Err(err) = rl.save_history(file_path) {
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

    if let Err(err) = rl.save_history(file_path) {
        println!("Error saving history: {:?}", err);
    }
}
