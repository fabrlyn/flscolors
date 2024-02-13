use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::many_m_n, sequence::tuple, IResult,
};

// TODO: Naming
pub struct Colors(Vec<Attribute>);

impl Colors {
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .flat_map(|c| [c.colors.0, c.colors.1])
            .map(|c| c.code())
            .collect()
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self(vec![
            Attribute {
                colors: ColorPair(Color::Blue, Color::Default),
                r#type: Type::Directory,
            },
            Attribute {
                colors: ColorPair(Color::Magenta, Color::Default),
                r#type: Type::SymbolicLink,
            },
            Attribute {
                colors: ColorPair(Color::Green, Color::Default),
                r#type: Type::Socket,
            },
            Attribute {
                colors: ColorPair(Color::Brown, Color::Default),
                r#type: Type::Pipe,
            },
            Attribute {
                colors: ColorPair(Color::Red, Color::Default),
                r#type: Type::Executable,
            },
            Attribute {
                colors: ColorPair(Color::Blue, Color::Cyan),
                r#type: Type::BlockSpecial,
            },
            Attribute {
                colors: ColorPair(Color::Blue, Color::Brown),
                r#type: Type::CharacterSpecial,
            },
            Attribute {
                colors: ColorPair(Color::Black, Color::Red),
                r#type: Type::ExecutableSetUid,
            },
            Attribute {
                colors: ColorPair(Color::Black, Color::Cyan),
                r#type: Type::ExecutableSetGid,
            },
            Attribute {
                colors: ColorPair(Color::Black, Color::Green),
                r#type: Type::DirectoryWithSticky,
            },
            Attribute {
                colors: ColorPair(Color::Black, Color::Brown),
                r#type: Type::DirectoryWithoutSticky,
            },
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
            Colors,
        )(input)
        .ok()
        .map(|(_, colors)| colors)
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
            Color::BoldCyan => "g",
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

pub enum UnderscoreColors {}
