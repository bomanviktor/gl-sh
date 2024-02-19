pub mod consts {
    use termion::color;
    use termion::color::Fg;

    pub const VERSION: &str = "0.1";

    pub const DIR_COLOR: Fg<color::Cyan> = Fg(color::Cyan);
    pub const EXECUTABLE_COLOR: Fg<color::Red> = Fg(color::Red);
    pub const SYMLINK_COLOR: Fg<color::Rgb> = Fg(color::Rgb(100, 0, 200));
}

pub mod scripts {
    pub mod for_in;
    pub mod gsh;
    pub mod if_else;
    pub mod interpreter;
    pub use interpreter::Interpreter;
}

pub mod commands {
    use std::env;
    pub fn get_absolute_path() -> String {
        env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .trim()
            .to_string()
    }

    pub fn get_parent() -> String {
        traverse_back("..")
    }

    pub fn traverse_back(arg: &str) -> String {
        let path = get_absolute_path();
        let split_path = path.split('/').collect::<Vec<&str>>();
        let n = split_path.len() - arg.matches("..").count();
        let mut new_path = split_path.split_at(n).0.join("/");

        if let Some((_, location)) = arg.rsplit_once("../") {
            new_path = format!("{new_path}/{location}");
        }

        if new_path.is_empty() {
            "/".to_string()
        } else {
            new_path
        }
    }

    pub fn traverse_home(arg: &str) -> String {
        let home_dir = dirs::home_dir().unwrap().to_string_lossy().to_string();
        let remaining_path = arg.replacen('~', "", 1);
        format!("{home_dir}{remaining_path}").trim().to_string()
    }

    pub mod cat;
    pub use cat::cat;

    pub mod cd;
    pub use cd::cd;

    pub mod clear;
    pub use clear::clear;

    pub mod cp;
    pub use cp::cp;

    pub mod date;
    pub use date::date;

    pub mod echo;
    pub use echo::echo;

    pub mod help;
    pub use help::help;

    pub mod ls;
    pub use ls::ls;

    pub mod mkdir;
    pub use mkdir::mkdir;

    pub mod mv;
    pub use mv::mv;

    pub mod pwd;
    pub use pwd::pwd;

    pub mod rm;
    pub use rm::rm;

    pub mod touch;
    pub use touch::touch;

    pub mod uname;
    pub use uname::uname;

    pub mod ver;
    pub use ver::ver;

    pub mod whoami;
    pub use whoami::who_am_i;
}

pub mod helpers {
    pub mod auto_complete;
    //pub use auto_complete::*;

    pub mod error;
    pub use error::command_error;
    pub mod execute;
    pub use execute::execute_commands;

    pub mod history;
    pub use history::add_to_history;

    pub mod parse;
    pub use parse::*;

    pub mod redirect;
    pub use redirect::redirect;

    pub mod prompt;
    pub use prompt::custom_prompt;
}
