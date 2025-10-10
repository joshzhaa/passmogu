use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Show,
    Save,
}

fn main() {
    let cli = Cli::parse();
    println!("{cli:?}");
    println!("Success!");
}
