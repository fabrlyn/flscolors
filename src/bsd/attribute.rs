use super::{color::Color, color_pair::ColorPair};

#[derive(Clone, Copy, Debug)]
pub enum Attribute {
    BlockSpecial,
    CharacterSpecial,
    Directory,
    DirectoryWithSticky,
    DirectoryWithoutSticky,
    Executable,
    ExecutableSetGid,
    ExecutableSetUid,
    Pipe,
    Socket,
    SymbolicLink,
}

impl Attribute {
    pub fn default_color_pair(&self) -> ColorPair {
        match self {
            Self::Directory => (Color::Blue, Color::Default).into(),
            Self::SymbolicLink => (Color::Magenta, Color::Default).into(),
            Self::Socket => (Color::Green, Color::Default).into(),
            Self::Pipe => (Color::Brown, Color::Default).into(),
            Self::Executable => (Color::Red, Color::Default).into(),
            Self::BlockSpecial => (Color::Blue, Color::Cyan).into(),
            Self::CharacterSpecial => (Color::Blue, Color::Brown).into(),
            Self::ExecutableSetUid => (Color::Black, Color::Red).into(),
            Self::ExecutableSetGid => (Color::Black, Color::Cyan).into(),
            Self::DirectoryWithSticky => (Color::Black, Color::Green).into(),
            Self::DirectoryWithoutSticky => (Color::Black, Color::Brown).into(),
        }
    }
}
