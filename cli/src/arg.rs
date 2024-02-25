use std::{
    io::{self, stdin},
    str::FromStr,
};

use flscolors::bsd;

#[derive(Clone, Copy, Debug)]
pub struct BsdColorsArg(bsd::Colors);

impl FromStr for BsdColorsArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        bsd::Colors::decode(s.trim())
            .ok_or("Not a valid LSCOLORS sequence.".to_string())
            .map(BsdColorsArg)
    }
}

impl From<BsdColorsArg> for bsd::Colors {
    fn from(value: BsdColorsArg) -> Self {
        value.0
    }
}

#[derive(Clone, Debug)]
pub enum StdinArg<A> {
    Arg(A),
    Stdin,
}

impl<A> StdinArg<A> {
    pub fn into_or_parse_line<P, E>(self, parser: P) -> Result<Result<A, E>, io::Error>
    where
        P: Fn(&str) -> Result<A, E>,
    {
        match self {
            StdinArg::Arg(arg) => Ok(Ok(arg)),
            StdinArg::Stdin => {
                let mut buffer = String::new();
                stdin().read_line(&mut buffer)?;
                Ok(parser(&buffer))
            }
        }
    }
}

impl<A, E> StdinArg<A>
where
    A: FromStr<Err = E>,
    E: Into<String>,
{
    pub fn parse(input: &str) -> Result<Self, String> {
        match input {
            "-" => Ok(Self::Stdin),
            input => A::from_str(input)
                .map(Self::Arg)
                .map_err(Into::into)
                .map_err(|e| format!("{e} Use '-' for piping from stdin.")),
        }
    }
}
