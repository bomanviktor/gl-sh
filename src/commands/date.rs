use crate::helpers::execute::ExecuteOption;
use crate::helpers::execute::ExecuteOption::Out;
use chrono::{DateTime, Datelike, Local, Timelike};

pub fn date() -> ExecuteOption {
    let current_time: DateTime<Local> = Local::now();

    let formatted_time = format!(
        "{} {} {:02} {:02}:{:02}:{:02} {} {}",
        current_time.format("%a"), // Abbreviated weekday name (e.g., Sat)
        current_time.format("%b"), // Abbreviated month name (e.g., Feb)
        current_time.day(),
        current_time.hour(),
        current_time.minute(),
        current_time.second(),
        current_time.format("%:z"), // Time zone abbreviation (e.g., EET)
        current_time.year()
    );

    Out(formatted_time)
}
