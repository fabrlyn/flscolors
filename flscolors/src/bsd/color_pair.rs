use nom::{combinator::map, sequence::tuple, IResult};

use super::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct ColorPair {
    pub foreground: Color,
    pub background: Color,
}

impl ColorPair {
    pub(super) fn parse_internal(input: &str) -> IResult<&str, Self> {
        map(
            tuple((Color::parse_internal, Color::parse_internal)),
            ColorPair::from,
        )(input)
    }
    
}

impl From<(Color, Color)> for ColorPair {
    fn from(value: (Color, Color)) -> Self {
        Self {
            foreground: value.0,
            background: value.1,
        }
    }
}
