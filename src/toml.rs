use std::error::Error;
use std::fs;
use toml::Table;

pub fn read(data_folder: &str, year: i32, month: u32) -> Result<(), Box<dyn Error>> {
    let path = &format!("{}/{}.{:02}.toml", data_folder, year, month);

    let data =
        fs::read_to_string(path).map_err(|err| format!("can't open file `{}`: {}", path, err))?;

    let data = data.parse::<Table>()?;

    Ok(())
}
