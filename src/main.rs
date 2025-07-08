mod cmdline;
mod date;
mod toml;

use std::error::Error;

fn main() {
    let data_folder = cmdline::parse();

    let (year, month) = date::get();

    let (income, expenditures) = toml::read(&data_folder, year, month);
}
