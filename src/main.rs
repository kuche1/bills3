mod cmdline;
mod date;

fn main() {
    let data_folder = cmdline::parse();

    println!("data_folder={}", data_folder);

    let (year, month) = date::get();

    println!("year={} month={}", year, month);
}
