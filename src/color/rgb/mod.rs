use super::convert::ConvertFrom;
use super::Color;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color for Rgb {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;
        
        let r = r | (r << 8);
        let g = g | (g << 8);
        let b = b | (b << 8);
        let a = 0xffff;
        
        (r, g, b, a)
    }
}

impl<C: Color> ConvertFrom<C> for Rgb {
    default fn convert_from(c: C) -> Rgb {
        let (r, g, b, _) = c.as_rgba();
        Rgb {
            r: ((r >> 8) & 0xff) as u8,
            g: ((g >> 8) & 0xff) as u8,
            b: ((b >> 8) & 0xff) as u8,
        }
    }
}
