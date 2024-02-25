// Reference to `ls`and `LSCOLORS`
// https://man.freebsd.org/cgi/man.cgi?query=ls&apropos=0&sektion=1&manpath=FreeBSD+15.0-CURRENT&arch=default&format=html
// Look at making some fns const if possible

pub mod attribute;
pub mod color;
pub mod color_pair;
pub mod colors;

pub use attribute::Attribute;
pub use color::Color;
pub use color_pair::ColorPair;
pub use colors::Colors;
