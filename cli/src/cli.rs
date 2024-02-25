use clap::{command, Args, Parser, Subcommand};
use flscolors::bsd::Colors;

#[derive(Debug, Parser)]
#[command(about = "Tool for ls colors", name = "flscolors", author = "fabrlyn")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(subcommand)]
    Bsd(Bsd),
}

#[derive(Debug, Subcommand)]
enum Bsd {
    Default,
}

#[derive(Args, Debug)]
#[command(about = "Print the default LSCOLORS sequence")]
struct Default {}

fn default() {
    println!("{}", Colors::default().to_string());
}

fn bsd(bsd: Bsd) {
    match bsd {
        Bsd::Default => default(),
    }
}

pub fn run() {
    match Cli::parse().command {
        Command::Bsd(command) => bsd(command),
    }
}
