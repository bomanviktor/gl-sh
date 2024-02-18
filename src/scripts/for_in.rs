use crate::helpers::execute_commands;
use crate::scripts::Interpreter;
use std::ops::Range;

impl Interpreter {
    pub fn for_in(&mut self, signature: String) {
        self.loop_var = loop_variable(&signature).to_string();
        for i in loop_range(&signature) {
            for input in loop_input(&signature) {
                let input = self.replace_variables(i, input);
                execute_commands(self, input.trim());
            }
        }
    }

    fn replace_variables(&self, i: usize, input: &str) -> String {
        let mut new_input = input.replace(&format!("${}", self.loop_var), &i.to_string());
        for (name, value) in &self.variables {
            new_input = new_input.replace(&format!("${name}"), &value.to_string());
        }
        new_input
    }
}

fn loop_variable(signature: &str) -> &str {
    signature.split_ascii_whitespace().nth(1).unwrap()
}

fn loop_range(signature: &str) -> Range<usize> {
    let range_split = signature
        .split_ascii_whitespace()
        .nth(3)
        .unwrap()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split("..")
        .collect::<Vec<&str>>();

    let start = range_split[0].parse::<usize>().unwrap();
    let end = range_split[1].parse::<usize>().unwrap();
    start..end
}

fn loop_input(signature: &str) -> Vec<&str> {
    signature
        .split_once('{')
        .unwrap()
        .1
        .trim()
        .trim_end_matches('}')
        .split('\n')
        .collect::<Vec<&str>>()
}
