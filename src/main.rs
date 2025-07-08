mod cmdline;
mod date;
mod toml;

fn main() {
    let mut data = vec![];

    {
        let data_folder = cmdline::parse();

        let (mut year, mut month) = date::get();

        loop {
            let d = match toml::read(&data_folder, year, month) {
                Some(v) => v,
                None => break,
            };
            data.push((year, month, d));

            month -= 1;
            if month <= 0 {
                year -= 1;
                month = 12;
            }
        }
    }

    let mut money = 0.0;

    for (year, month, (income, expenditures)) in data.into_iter().rev() {
        money += income;

        for (day_idx, day_expenditure) in expenditures.iter().enumerate() {
            let day = day_idx + 1;

            money -= day_expenditure;

            println!(
                "{}.{:02}.{:02} -> {:6.2} -> {:7.2}",
                year, month, day, day_expenditure, money
            );
        }
    }
}
