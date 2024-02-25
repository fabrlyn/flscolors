use std::str::FromStr;

use clap::{Args, Subcommand};
use flscolors::bsd::{self, colors::IntoColorsWithDefault, Color};

use crate::{
    arg::{BsdColorsArg, StdinArg},
    util::GetOrExit,
};

#[derive(Debug, Subcommand)]
pub enum Bsd {
    Default,
    Attributes,
    Colors,
    Print(Print),
}

#[derive(Args, Debug)]
#[command()]
struct Default {}

// Print colors with/without actually coloring them
#[derive(Args, Debug)]
#[command()]
struct Colors {}

// Print attributes with/without index
#[derive(Args, Debug)]
#[command()]
struct Attributes {}

#[derive(Args, Debug)]
#[command()]
pub struct Print {
    #[arg(value_parser = StdinArg::<BsdColorsArg>::parse)]
    colors: StdinArg<BsdColorsArg>,
    // TODO: Print with the colors
    // TODO: Show the how it maps to attribute Directory: Blue(f)/Default(x)
}

fn default() {
    println!("{}", bsd::Colors::default().to_string());
}

fn print(print: Print) {
    let colors: bsd::Colors = print
        .colors
        .into_or_parse_line(BsdColorsArg::from_str)
        .get_or_exit()
        .into();

    println!(
        "{}\n{}Directory",
        colors.directory.foreground.code(),
        colors.directory.background.code()
    );
    println!(
        "{}\n{}Symbolic link",
        colors.symbolic_link.foreground.code(),
        colors.symbolic_link.background.code()
    );
    println!(
        "{}\n{}Socket",
        colors.socket.foreground.code(),
        colors.socket.background.code()
    );
    println!(
        "{}\n{}Pipe",
        colors.pipe.foreground.code(),
        colors.pipe.background.code()
    );
    println!(
        "{}\n{}Executable",
        colors.executable.foreground.code(),
        colors.executable.background.code()
    );
    println!(
        "{}\n{}Block special",
        colors.block_special.foreground.code(),
        colors.block_special.background.code()
    );
    println!(
        "{}\n{}Character special",
        colors.character_special.foreground.code(),
        colors.character_special.background.code()
    );
    println!(
        "{}\n{}Character special",
        colors.character_special.foreground.code(),
        colors.character_special.background.code()
    );
    println!(
        "{}\n{}Executable with setuid bit set",
        colors.executable_set_uid.foreground.code(),
        colors.executable_set_uid.background.code()
    );
    println!(
        "{}\n{}Executable with setgid bit set",
        colors.executable_set_gid.foreground.code(),
        colors.executable_set_gid.background.code()
    );
    println!(
        "{}\n{}Directory with sticky bit",
        colors.directory_with_sticky.foreground.code(),
        colors.directory_with_sticky.background.code()
    );
    println!(
        "{}\n{}Directory without sticky bit",
        colors.directory_without_sticky.foreground.code(),
        colors.directory_without_sticky.background.code()
    );
}

// TODO: Make this prettier
fn colors() {
    println!("{} Black\n{} Red\n{} Green\n{} Brown\n{} Blue\n{} Magenta\n{} Cyan\n{} Light Grey\nx Default{}\n{} Bold Black\n{} Bold red\n{} Bold green\n{} Bold brown\n{} Bold blue\n{} Bold magenta\n{} Bold cyan\n{} Bold light grey",
Color::Black.code(), Color::Red.code(), Color::Green.code(), Color::Brown.code(), Color::Blue.code(), Color::Magenta.code(), Color::Cyan.code(), Color::LightGrey.code(), Color::Default.code(), Color::BoldBlack.code(), Color::BoldRed.code(),
Color::BoldGreen.code(), Color::BoldBrown.code(), Color::BoldBlue.code(), Color::BoldMagenta.code(), Color::BoldCyan.code(), Color::BoldLightGrey.code()

);
}

fn attributes() {
    println!("1. Directory\n2. Symbolic link\n3. Socket\n4. Pipe\n5. Executable\n6. Block special\n7. Character special\n8. Executable with setuid bit set\n9. Executable with setgid bit set\n10. Directory with sticky bit\n11. Directory without sticky bit");
}

pub fn run(bsd: Bsd) {
    match bsd {
        Bsd::Default => default(),
        Bsd::Print(command) => print(command),
        Bsd::Attributes => attributes(),
        Bsd::Colors => colors(),
    }
}
