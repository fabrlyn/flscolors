// Reference to `ls`and `LSCOLORS`
// https://man.freebsd.org/cgi/man.cgi?query=ls&apropos=0&sektion=1&manpath=FreeBSD+15.0-CURRENT&arch=default&format=html

use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::many_m_n, sequence::tuple, IResult,
};

// TODO: Naming
pub struct Colors([Attribute; 11]);

impl Colors {
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .flat_map(|c| [c.colors.0, c.colors.1])
            .map(|c| c.code())
            .collect()
    }
}

impl Type {
    const fn default_colors(&self) -> ColorPair {
        match self {
            Type::BlockSpecial => ColorPair(Color::Blue, Color::Cyan),
            Type::CharacterSpecial => ColorPair(Color::Blue, Color::Brown),
            Type::Directory => ColorPair(Color::Blue, Color::Default),
            Type::DirectoryWithSticky => ColorPair(Color::Black, Color::Green),
            Type::DirectoryWithoutSticky => ColorPair(Color::Black, Color::Brown),
            Type::Executable => ColorPair(Color::Red, Color::Default),
            Type::ExecutableSetGid => ColorPair(Color::Black, Color::Cyan),
            Type::ExecutableSetUid => ColorPair(Color::Black, Color::Red),
            Type::Pipe => ColorPair(Color::Brown, Color::Default),
            Type::Socket => ColorPair(Color::Green, Color::Default),
            Type::SymbolicLink => ColorPair(Color::Magenta, Color::Default),
        }
    }
}

impl Attribute {
    fn default_from_type(r#type: Type) -> Attribute {
        Attribute {
            colors: r#type.default_colors(),
            r#type,
        }
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self([
            Attribute::default_from_type(Type::Directory),
            Attribute::default_from_type(Type::SymbolicLink),
            Attribute::default_from_type(Type::Socket),
            Attribute::default_from_type(Type::Pipe),
            Attribute::default_from_type(Type::Executable),
            Attribute::default_from_type(Type::BlockSpecial),
            Attribute::default_from_type(Type::CharacterSpecial),
            Attribute::default_from_type(Type::ExecutableSetUid),
            Attribute::default_from_type(Type::ExecutableSetGid),
            Attribute::default_from_type(Type::DirectoryWithSticky),
            Attribute::default_from_type(Type::DirectoryWithoutSticky),
        ])
    }
}

impl Colors {
    pub fn decode(input: &str) -> Option<Colors> {
        let mut position = 11;
        map(
            many_m_n(11, 11, move |input| {
                let result = Attribute::parse_internal(input, position);
                position = position + 1;
                result
            }),
            |colors| colors.try_into().map(Colors),
        )(input)
        .ok()
        .and_then(|x| x.1.ok())
    }
}

// TODO: Naming
#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    Blue,
    BoldBlack,
    BoldBlue,
    BoldBrown,
    BoldCyan,
    BoldDefault,
    BoldGreen,
    BoldLightGrey,
    BoldMagenta,
    BoldRed,
    Brown,
    Cyan,
    Default,
    Green,
    LightGrey,
    Magenta,
    Red,
}

// TODO: Naming
#[derive(Clone, Copy, Debug)]
pub enum Type {
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

#[derive(Clone, Copy, Debug)]
pub struct Attribute {
    colors: ColorPair,
    r#type: Type,
}

#[derive(Clone, Copy, Debug)]
pub struct ColorPair(Color, Color);

impl From<(Color, Color)> for ColorPair {
    fn from(value: (Color, Color)) -> Self {
        Self(value.0, value.1)
    }
}

impl ColorPair {
    fn parse_internal(input: &str) -> IResult<&str, Self> {
        map(
            tuple((Color::parse_internal, Color::parse_internal)),
            ColorPair::from,
        )(input)
    }
}

impl Attribute {
    fn parse_internal(input: &str, position: u8) -> IResult<&str, Attribute> {
        let (rest, color_pair) = ColorPair::parse_internal(input)?;

        // TODO: Clean this up
        let r#type = Type::from_position(position).ok_or_else(|| {
            nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail))
        })?;

        Ok((rest, Attribute::from((color_pair, r#type))))
    }
}

impl From<(ColorPair, Type)> for Attribute {
    fn from((colors, r#type): (ColorPair, Type)) -> Self {
        Self { colors, r#type }
    }
}

impl Color {
    pub const fn code(&self) -> &'static str {
        match self {
            Color::Black => "a",
            Color::Blue => "e",
            Color::BoldBlack => "A",
            Color::BoldBlue => "E",
            Color::BoldBrown => "D",
            Color::BoldCyan => "G",
            Color::BoldDefault => "X",
            Color::BoldGreen => "C",
            Color::BoldLightGrey => "H",
            Color::BoldMagenta => "F",
            Color::BoldRed => "B",
            Color::Brown => "d",
            Color::Cyan => "g",
            Color::Default => "x",
            Color::Green => "c",
            Color::LightGrey => "h",
            Color::Magenta => "f",
            Color::Red => "b",
        }
    }

    fn parse_internal(input: &str) -> IResult<&str, Color> {
        alt((
            map(tag(Color::Black.code()), |_| Color::Black),
            map(tag(Color::Blue.code()), |_| Color::Blue),
            map(tag(Color::BoldBlack.code()), |_| Color::BoldBlack),
            // TODO: Fill out this
        ))(input)
    }
}

impl Type {
    pub fn from_position(position: u8) -> Option<Self> {
        match position {
            1 => Some(Type::Directory),
            2 => Some(Type::SymbolicLink),
            3 => Some(Type::Socket),
            4 => Some(Type::Pipe),
            5 => Some(Type::Executable),
            6 => Some(Type::BlockSpecial),
            7 => Some(Type::CharacterSpecial),
            8 => Some(Type::ExecutableSetUid),
            9 => Some(Type::ExecutableSetGid),
            10 => Some(Type::DirectoryWithSticky),
            11 => Some(Type::DirectoryWithoutSticky),
            _ => None,
        }
    }
}

pub struct Builder {
    directory: ColorPair,
    symbolic_link: ColorPair,
    socket: ColorPair,
    pipe: ColorPair,
    executable: ColorPair,
    block_special: ColorPair,
    character_special: ColorPair,
    executable_set_uid: ColorPair,
    executable_set_gid: ColorPair,
    directory_with_sticky: ColorPair,
    directory_without_sticky: ColorPair,
}
