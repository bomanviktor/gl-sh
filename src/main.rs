use gritlab_shell::commands::traverse_home;
use gritlab_shell::helpers::auto_complete::CustomHelper;
use gritlab_shell::helpers::history::{init_history, HISTORY_PATH};
use gritlab_shell::helpers::{custom_prompt, execute_commands};
use gritlab_shell::scripts::Interpreter;
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;
use std::env;

fn main() {
    let mut editor = Editor::<CustomHelper, FileHistory>::new().unwrap();
    let filename_completer = FilenameCompleter::new();
    let custom_helper = CustomHelper { filename_completer };
    editor.set_helper(Some(custom_helper));

    init_history();
    let file_path = format!("{}/{}", traverse_home(""), HISTORY_PATH);
    if editor.load_history(&file_path).is_err() {
        println!("Failed to load history. Exiting gl-sh...");
        return;
    }

    let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
    env::set_current_dir(&current_dir).expect("Could not start the shell.");

    let mut interpreter = Interpreter::new(current_dir.clone());
    loop {
        let readline = editor.readline(custom_prompt().as_str());
        match readline {
            Ok(line) => {
                editor.add_history_entry(line.as_str()).unwrap();
                if !execute_commands(&mut interpreter, line.trim()) {
                    if let Err(err) = editor.save_history(&file_path) {
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

    if let Err(err) = editor.save_history(&file_path) {
        println!("Error saving history: {:?}", err);
    }
}
