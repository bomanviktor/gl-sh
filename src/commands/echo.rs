use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Out;

pub fn echo(args: Vec<&str>) -> ExecuteOption {
    Out(args.join(" "))
}
