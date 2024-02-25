use flscolors::bsd::{Color, Colors};

fn main() {
    let colors = Colors::default().to_string();
    println!("{colors}");
    //         "exfxcxdxbxegedabagacad"
    assert_eq!("exfxcxdxbxegedabagacad", colors);

    let colors = Colors::default()
        .with_directory(Color::Green)
        .with_pipe(Color::Magenta)
        .with_socket(Color::Cyan);

    println!("-- changed colors --");
    println!("{}", colors.to_string());
}
