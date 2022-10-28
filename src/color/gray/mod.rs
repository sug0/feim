use super::convert::ConvertFrom;
use super::Color;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Gray {
    pub y: u8,
}

impl Color for Gray {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        let y = self.y as u32;
        let y = y | (y << 8);

        let r = y;
        let g = y;
        let b = y;
        let a = 0xffff;

        (r, g, b, a)
    }
}

impl<C: Color> ConvertFrom<C> for Gray {
    fn convert_from(c: C) -> Gray {
        let (r, g, b, _) = c.as_rgba();
        let y = ((19595 * r + 38470 * g + 7471 * b + 0x8000) >> 24) as u8;
        Gray { y }
    }
}

impl From<Gray> for u8 {
    fn from(c: Gray) -> u8 {
        c.y
    }
}

impl From<u8> for Gray {
    fn from(y: u8) -> Gray {
        Gray { y }
    }
}
