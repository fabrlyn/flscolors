use clap::{command, Args, Parser, Subcommand};

use crate::{
    arg::{BsdColorsArg, StdinArg},
    bsd::{self, Bsd},
};

#[derive(Debug, Parser)]
#[command(name = "flscolors", author = "fabrlyn")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(subcommand)]
    Bsd(Bsd),
}

pub fn run() {
    match Cli::parse().command {
        Command::Bsd(command) => bsd::run(command),
    }
}
