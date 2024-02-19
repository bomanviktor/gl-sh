use std::collections::HashMap;

pub struct Interpreter {
    pub last_location: Vec<String>,
    pub variables: HashMap<String, String>,
    pub inside_loop: bool,
    pub loop_var: String,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            last_location: Vec::new(),
            variables: HashMap::new(),
            inside_loop: false,
            loop_var: String::new(),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}
