use chrono::Datelike;
use std::error::Error;
use std::fs;
use toml::Table;
use toml::Value;

///////////// vvvvv stupid fucking shit (this should have been included in the library) // https://github.com/chronotope/chrono/issues/69
trait NaiveDateExt {
    fn days_in_month(&self) -> u32;
    fn is_leap_year(&self) -> bool;
}

impl NaiveDateExt for chrono::NaiveDate {
    fn days_in_month(&self) -> u32 {
        let month = self.month();
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month: {}", month),
        }
    }

    fn is_leap_year(&self) -> bool {
        let year = self.year();
        return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }
}
///////////// ^^^^^

fn value_sum(value: Value) -> f32 {
    match value {
        Value::Integer(v) => return v as f32,

        Value::Float(v) => return v as f32,

        Value::Array(v) => {
            let mut sum: f32 = 0.0;
            for value in v {
                sum += value_sum(value);
            }
            return sum;
        }

        Value::Table(v) => {
            let mut sum: f32 = 0.0;
            for (_key, value) in v {
                sum += value_sum(value);
            }
            return sum;
        }

        _ => {
            panic!("unsupported value: `{value}`");
        }
    }
}

pub fn read(data_folder: &str, year: i32, month: u32) -> (f32, Vec<f32>) {
    let date = chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let days_in_month = date.days_in_month();

    let path = &format!("{}/{}.{:02}.toml", data_folder, year, month);

    let data = fs::read_to_string(path)
        .map_err(|err| panic!("can't open file `{}`: {}", path, err))
        .ok()
        .unwrap();

    let data = data.parse::<Table>().ok().unwrap();

    let mut income: f32 = 0.0;
    let mut expenditures_monthly: f32 = 0.0;
    let mut expenditures_regular = vec![0.0_f32; days_in_month as usize];

    for (item_name, item_value) in data {
        match item_name.as_str() {
            "INCOME" => {
                income += value_sum(item_value);
            }

            "EXPENDITURES-MONTHLY" => {
                expenditures_monthly += value_sum(item_value);
            }

            "EXPENDITURES-REGULAR" => match item_value {
                Value::Table(value) => {
                    for (day, money) in value {
                        let day: usize = day
                            .parse()
                            .map_err(|_| panic!("invalid day of month: `{day}`"))
                            .ok()
                            .unwrap();

                        let idx = day
                            .checked_add_signed(-1)
                            .ok_or_else(|| panic!("first day of month is 1"))
                            .ok()
                            .unwrap();

                        let money = value_sum(money);

                        match expenditures_regular.get_mut(idx) {
                            None => panic!("invalid day of month: `{}`", day),
                            Some(item) => *item += money,
                        }
                    }
                }

                _ => {
                    panic!("unsupported item value: `{item_value}`");
                }
            },

            _ => {
                panic!("unknown item name: `{}`", item_name);
            }
        }
    }

    income -= expenditures_monthly;

    (income, expenditures_regular)
}
