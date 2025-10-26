mod error;
// supplies passmogu unlock
mod session;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,
    Unlock,
    Store,
}

fn main() -> Result<(), error::Error> {
    let args = Args::parse();
    match args.command {
        Command::Init => todo!(),
        Command::Unlock => session::session_repl(),
        Command::Store => todo!(),
    }
}
