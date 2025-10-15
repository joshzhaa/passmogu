use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Show,
    Save,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
    println!("{:?}", args.command);
    println!("Success!");
}
