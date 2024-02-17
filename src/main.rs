use flscolors::bsd::{builder, Color, Colors};

fn main() {
    let colors = Colors::default().to_string();
    println!("{colors}");
    assert_eq!("exfxcxdxbxegedabagacad", colors);

    let colors = builder()
        .directory(Color::Green)
        .pipe(Color::Magenta)
        .socket(Color::Cyan)
        .build();
    println!("-- via builder --");
    println!("{}", colors.to_string());
}
