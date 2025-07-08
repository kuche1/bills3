mod cmdline;

fn main() {
    let data_folder = cmdline::main();

    println!("data_folder={}", data_folder);
}
