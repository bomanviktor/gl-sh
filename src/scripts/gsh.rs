use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::{Empty, Exit};
use crate::helpers::{command_error, execute_commands};
use crate::scripts::Interpreter;
use std::fs;

impl Interpreter {
    pub fn gsh(&mut self, input: String) -> ExecuteOption {
        let script_file = match fs::read(&input) {
            Ok(f) => f,
            Err(e) => {
                command_error("gsh", e, &input.clone());
                return Empty;
            }
        };
        let script_file = String::from_utf8_lossy(&script_file);
        let script_file = script_file.split('\n').collect::<Vec<&str>>();

        for (i, row) in script_file.iter().enumerate() {
            match row
                .split_ascii_whitespace()
                .next()
                .unwrap_or_default()
                .trim()
            {
                "for" => {
                    let signature = parse_loop(&script_file[i..]);
                    self.inside_loop = true;
                    self.for_in(signature);
                }
                "}" => self.inside_loop = false,
                _ => {
                    if self.inside_loop {
                        continue;
                    }
                    if !execute_commands(self, row.trim()) {
                        return Exit;
                    }
                }
            }
        }
        Empty
    }
}

fn parse_loop(rest_of_script: &[&str]) -> String {
    let mut parsed_loop = String::new();
    for row in rest_of_script {
        parsed_loop.push_str(row);
        if row.contains('}') {
            break;
        }
    }
    parsed_loop
}
