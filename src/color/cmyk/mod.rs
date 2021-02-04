use super::Color;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Cmyk {
    pub c: u8,
    pub m: u8,
    pub y: u8,
    pub k: u8,
}
