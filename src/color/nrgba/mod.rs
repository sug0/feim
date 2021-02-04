use super::Color;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Nrgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color for Nrgba {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;
        let a = self.a as u32;
        
        let r = r | (r << 8);
        let g = g | (g << 8);
        let b = b | (b << 8);
        let a = a | (a << 8);
        
        (r, g, b, a)
    }
}

impl<C: Color> From<&C> for Nrgba {
    fn from(c: &C) -> Nrgba {
        let (r, g, b, a) = c.as_rgba();
        Nrgba {
            r: ((r >> 8) & 0xff) as u8,
            g: ((g >> 8) & 0xff) as u8,
            b: ((b >> 8) & 0xff) as u8,
            a: ((a >> 8) & 0xff) as u8,
        }
    }
}

impl From<Nrgba> for u32 {
    fn from(c: Nrgba) -> u32 {
        let r = (c.r as u32) << (8 * 0);
        let g = (c.g as u32) << (8 * 1);
        let b = (c.b as u32) << (8 * 2);
        let a = (c.a as u32) << (8 * 3);
        r | g | b | a
    }
}
