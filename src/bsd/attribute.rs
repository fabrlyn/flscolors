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
    pub fn from_position(position: u8) -> Option<Self> {
        match position {
            1 => Some(Attribute::Directory),
            2 => Some(Attribute::SymbolicLink),
            3 => Some(Attribute::Socket),
            4 => Some(Attribute::Pipe),
            5 => Some(Attribute::Executable),
            6 => Some(Attribute::BlockSpecial),
            7 => Some(Attribute::CharacterSpecial),
            8 => Some(Attribute::ExecutableSetUid),
            9 => Some(Attribute::ExecutableSetGid),
            10 => Some(Attribute::DirectoryWithSticky),
            11 => Some(Attribute::DirectoryWithoutSticky),
            _ => None,
        }
    }

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
