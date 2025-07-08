use chrono::{Datelike, Local};

pub fn get() -> (i32, u32) {
    let now = Local::now();
    let year = now.year();
    let month = now.month();

    (year, month)
}
