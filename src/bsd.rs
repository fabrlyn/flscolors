// Reference to `ls`and `LSCOLORS`
// https://man.freebsd.org/cgi/man.cgi?query=ls&apropos=0&sektion=1&manpath=FreeBSD+15.0-CURRENT&arch=default&format=html
// Look at making some fns const if possible

use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::many_m_n, sequence::tuple, IResult,
};

// TODO: Naming
pub struct Colors {
    block_special: ColorPair,
    character_special: ColorPair,
    directory: ColorPair,
    directory_with_sticky: ColorPair,
    directory_without_sticky: ColorPair,
    executable: ColorPair,
    executable_set_gid: ColorPair,
    executable_set_uid: ColorPair,
    pipe: ColorPair,
    socket: ColorPair,
    symbolic_link: ColorPair,
}

impl Colors {
    pub fn to_string(&self) -> String {
        self.as_sequence()
            .iter()
            .flat_map(|c| [c.foreground, c.background])
            .map(|c| c.code())
            .collect()
    }

    fn as_sequence(&self) -> [ColorPair; 11] {
        [
            self.directory,
            self.symbolic_link,
            self.socket,
            self.pipe,
            self.executable,
            self.block_special,
            self.character_special,
            self.executable_set_uid,
            self.executable_set_gid,
            self.directory_with_sticky,
            self.directory_without_sticky,
        ]
    }
}

impl Type {
    fn default_colors(&self) -> ColorPair {
        match self {
            Type::BlockSpecial => (Color::Blue, Color::Cyan).into(),
            Type::CharacterSpecial => (Color::Blue, Color::Brown).into(),
            Type::Directory => (Color::Blue, Color::Default).into(),
            Type::DirectoryWithSticky => (Color::Black, Color::Green).into(),
            Type::DirectoryWithoutSticky => (Color::Black, Color::Brown).into(),
            Type::Executable => (Color::Red, Color::Default).into(),
            Type::ExecutableSetGid => (Color::Black, Color::Cyan).into(),
            Type::ExecutableSetUid => (Color::Black, Color::Red).into(),
            Type::Pipe => (Color::Brown, Color::Default).into(),
            Type::Socket => (Color::Green, Color::Default).into(),
            Type::SymbolicLink => (Color::Magenta, Color::Default).into(),
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
pub struct ColorPair {
    foreground: Color,
    background: Color,
}

impl From<(Color, Color)> for ColorPair {
    fn from((foreground, background): (Color, Color)) -> Self {
        Self {
            foreground,
            background,
        }
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
    pub fn code(&self) -> &'static str {
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

// TODO: Naming
pub trait ColorsOrDefault: Sized {
    fn colors(self, colors: ColorPair) -> ColorPair;
}

impl ColorsOrDefault for Color {
    fn colors(self, colors: ColorPair) -> ColorPair {
        (self, colors.background).into()
    }
}

impl ColorsOrDefault for ColorPair {
    fn colors(self, colors: ColorPair) -> ColorPair {
        self
    }
}

impl ColorsOrDefault for (Color, Color) {
    fn colors(self, colors: ColorPair) -> ColorPair {
        (self.0, self.1).into()
    }
}

impl Builder {
    pub fn directory<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.directory = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn block_special<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.block_special = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn character_special<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.character_special = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn directory_with_sticky<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.directory_with_sticky = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn directory_without_sticky<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.directory_without_sticky = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn executable<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.executable = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn executable_set_gid<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.executable_set_gid = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn executable_set_uid<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.executable_set_uid = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn pipe<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.pipe = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn socket<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.socket = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn symbolic_link<C>(mut self, colors: C) -> Builder
    where
        C: ColorsOrDefault,
    {
        self.symbolic_link = colors.colors(Type::Directory.default_colors());
        self
    }

    pub fn build(self) -> Colors {
        Colors([
            self.directory,
            self.symbolic_link,
            self.socket,
            self.pipe,
            self.executable,
            self.block_special,
            self.character_special,
            self.executable_set_uid,
            self.executable_set_gid,
            self.directory_with_sticky,
            self.directory_without_sticky,
        ])
    }
}

pub fn builder() -> Builder {
    Builder {
        block_special: Type::BlockSpecial.default_colors(),
        character_special: Type::CharacterSpecial.default_colors(),
        directory: Type::Directory.default_colors(),
        directory_with_sticky: Type::DirectoryWithSticky.default_colors(),
        directory_without_sticky: Type::DirectoryWithoutSticky.default_colors(),
        executable: Type::Executable.default_colors(),
        executable_set_gid: Type::ExecutableSetGid.default_colors(),
        executable_set_uid: Type::ExecutableSetUid.default_colors(),
        pipe: Type::Pipe.default_colors(),
        socket: Type::Socket.default_colors(),
        symbolic_link: Type::SymbolicLink.default_colors(),
    }
}
