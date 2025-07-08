mod cmdline;
mod date;
mod toml;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data_folder = cmdline::parse();

    let (year, month) = date::get();

    let toml_data = toml::read(&data_folder, year, month)?;

    Ok(())
}
