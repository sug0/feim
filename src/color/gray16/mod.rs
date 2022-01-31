use super::convert::ConvertFrom;
use super::Color;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Gray16 {
    pub y: u16,
}

impl Color for Gray16 {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        let y = self.y as u32;

        let r = y;
        let g = y;
        let b = y;
        let a = 0xffff;
        
        (r, g, b, a)
    }
}

impl<C: Color> ConvertFrom<C> for Gray16 {
    default fn convert_from(c: C) -> Gray16 {
        let (r, g, b, _) = c.as_rgba();
        let y = ((19595*r + 38470*g + 7471*b + 0x8000) >> 16) as u16;
        Gray16 { y }
    }
}

impl From<Gray16> for u16 {
    fn from(c: Gray16) -> u16 {
        c.y
    }
}

impl From<u16> for Gray16 {
    fn from(y: u16) -> Gray16 {
        Gray16 { y }
    }
}
