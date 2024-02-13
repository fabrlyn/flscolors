use flscolors::Colors;

fn main() {
    let colors = Colors::default().to_string();
    println!("{colors}");
    assert_eq!("exfxcxdxbxegedabagacad", colors);
}
