use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Empty;
use std::io;
use std::io::Write;

pub fn clear() -> ExecuteOption {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
    Empty
}
