use clap::Parser; // cargo add clap --features derive

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder containing toml files
    #[arg(short, long)]
    data_folder: String,
}

pub fn main() -> String {
    let args = Args::parse();
    args.data_folder
}
