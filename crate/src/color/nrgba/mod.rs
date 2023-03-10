use super::convert::ConvertFrom;
use super::{Color, Zero};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Nrgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Zero for Nrgba {
    const ZERO: Self = Nrgba {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
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

impl<C: Color> ConvertFrom<C> for Nrgba {
    fn convert_from(c: C) -> Nrgba {
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
        let r = c.r as u32;
        let g = (c.g as u32) << 8;
        let b = (c.b as u32) << (8 * 2);
        let a = (c.a as u32) << (8 * 3);
        r | g | b | a
    }
}

impl From<u32> for Nrgba {
    fn from(c: u32) -> Nrgba {
        let r = (c & 0xff) as u8;
        let g = ((c & 0xff00) >> 8) as u8;
        let b = ((c & 0xff0000) >> 16) as u8;
        let a = ((c & 0xff000000) >> 24) as u8;
        Nrgba { r, g, b, a }
    }
}
