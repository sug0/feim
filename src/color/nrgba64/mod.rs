use super::Color;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Nrgba64 {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
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
