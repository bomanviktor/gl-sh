use crate::consts::VERSION;
use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Out;
use termion::color;

pub fn ver(_flags: Vec<&str>) -> ExecuteOption {
    let version = format!(
        "Version: {}{}{}",
        color::Fg(color::LightYellow),
        VERSION,
        color::Fg(color::Reset)
    );
    Out(version)
}
