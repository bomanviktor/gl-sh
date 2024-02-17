use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Out;

pub fn echo(args: String) -> ExecuteOption {
    let args = args
        .split_ascii_whitespace()
        .skip_while(|a| a.starts_with('-'))
        .collect::<Vec<&str>>();
    Out(args.join(" "))
}
