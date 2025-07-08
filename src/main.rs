mod cmdline;

fn main() {
    let data_folder = cmdline::parse();

    println!("data_folder={}", data_folder);
}
