mod rgb;
mod cmyk;
mod nrgba;
mod nrgba64;
mod gray;
mod gray16;

pub use rgb::*;
pub use cmyk::*;
pub use nrgba::*;
pub use nrgba64::*;
pub use gray::*;
pub use gray16::*;

pub trait Color {
    /// Return an alpha premultiplied color in RGBA 16 bits.
    fn as_rgba(&self) -> (u32, u32, u32, u32);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Nrgba64 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
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

impl Color for Nrgba64 {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;
        let a = self.a as u32;
        
        let r = (r * a) / 0xffff;
        let g = (g * a) / 0xffff;
        let b = (b * a) / 0xffff;
        
        (r, g, b, a)
    }
}
