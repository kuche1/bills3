mod cmdline;
mod date;
mod toml;

use std::error::Error;

fn main() {
    let data_folder = cmdline::parse();

    let (year, month) = date::get();

    let mut money = 0.0;

    let (income, expenditures) = toml::read(&data_folder, year, month);

    money += income;

    for (day_idx, day_expenditure) in expenditures.iter().enumerate() {
        let day = day_idx + 1;

        money -= day_expenditure;

        println!(
            "{:02}.{:02} -> {:5.2} -> {:.2}",
            month, day, day_expenditure, money
        );
    }
}
