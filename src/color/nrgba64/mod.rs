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

impl<C: Color> From<&C> for Nrgba64 {
    fn from(c: &C) -> Nrgba64 {
        let (r, g, b, a) = c.as_rgba();
        Nrgba64 {
            r: (r & 0xffff) as u16,
            g: (g & 0xffff) as u16,
            b: (b & 0xffff) as u16,
            a: (a & 0xffff) as u16,
        }
    }
}

impl From<Nrgba64> for u64 {
    fn from(c: Nrgba64) -> u64 {
        let r = (c.r as u64) << (16 * 0);
        let g = (c.g as u64) << (16 * 1);
        let b = (c.b as u64) << (16 * 2);
        let a = (c.a as u64) << (16 * 3);
        r | g | b | a
    }
}
