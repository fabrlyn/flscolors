use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

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

    pub(super) fn parse_internal(input: &str) -> IResult<&str, Color> {
        alt((
            map(tag(Color::Black.code()), |_| Color::Black),
            map(tag(Color::Blue.code()), |_| Color::Blue),
            map(tag(Color::BoldBlack.code()), |_| Color::BoldBlack),
            // TODO: Fill out this
        ))(input)
    }
}
