use crate::commands::get_absolute_path;
use std::collections::HashMap;

pub struct Interpreter {
    pub last_location: Vec<String>,
    pub variables: HashMap<String, String>,
    pub inside_loop: bool,
    pub loop_var: String,
}

impl Interpreter {
    pub fn new(location: String) -> Self {
        Self {
            last_location: vec![location],
            variables: HashMap::new(),
            inside_loop: false,
            loop_var: String::new(),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new(get_absolute_path())
    }
}
