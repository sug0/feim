#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Nrgba64 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Nrgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Cmyk {
    pub c: u8,
    pub m: u8,
    pub y: u8,
    pub k: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Gray {
    pub y: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Gray16 {
    pub y: u16,
}
