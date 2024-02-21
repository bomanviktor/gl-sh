use crate::commands::{cd, get_absolute_path};
use crate::helpers::execute::ExecuteOption::Out;
use dirs::home_dir;
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

    pub fn cd_last_location(&mut self) {
        let last_location = self
            .last_location
            .iter()
            .cloned()
            .nth_back(1)
            .unwrap_or_default();
        if let Out(path) = cd(vec![&last_location], true) {
            let home_dir = home_dir().unwrap().to_string_lossy().to_string();
            println!("{}", path.replace(&home_dir, "~"));
            self.last_location.push(path);
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new(get_absolute_path())
    }
}
