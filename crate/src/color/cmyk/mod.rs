use super::convert::ConvertFrom;
use super::Color;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Cmyk {
    pub c: u8,
    pub m: u8,
    pub y: u8,
    pub k: u8,
}

impl Color for Cmyk {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        let c = self.c as u32;
        let m = self.m as u32;
        let y = self.y as u32;
        let k = self.k as u32;

        let w = 0xffff - k * 0x101;

        let r = (0xffff - c * 0x101) * w / 0xffff;
        let g = (0xffff - m * 0x101) * w / 0xffff;
        let b = (0xffff - y * 0x101) * w / 0xffff;
        let a = 0xffff;

        (r, g, b, a)
    }
}

impl<C: Color> ConvertFrom<C> for Cmyk {
    fn convert_from(c: C) -> Cmyk {
        let (r, g, b, _) = c.as_rgba();

        let w = std::cmp::min(std::cmp::min(r, g), b);

        if w == 0 {
            return Cmyk {
                c: 0,
                m: 0,
                y: 0,
                k: 0xff,
            };
        }

        let c = (w - r) * 0xffff / w;
        let m = (w - g) * 0xffff / w;
        let y = (w - b) * 0xffff / w;
        let k = 0xffff - w;

        Cmyk {
            c: ((c >> 8) & 0xff) as u8,
            m: ((m >> 8) & 0xff) as u8,
            y: ((y >> 8) & 0xff) as u8,
            k: ((k >> 8) & 0xff) as u8,
        }
    }
}

impl From<Cmyk> for u32 {
    fn from(cmyk: Cmyk) -> u32 {
        let c = cmyk.c as u32;
        let m = (cmyk.m as u32) << 8;
        let y = (cmyk.y as u32) << (8 * 2);
        let k = (cmyk.k as u32) << (8 * 3);
        c | m | y | k
    }
}

impl From<u32> for Cmyk {
    fn from(cmyk: u32) -> Cmyk {
        let c = (cmyk & 0xff) as u8;
        let m = ((cmyk & 0xff00) >> 8) as u8;
        let y = ((cmyk & 0xff0000) >> 16) as u8;
        let k = ((cmyk & 0xff000000) >> 24) as u8;
        Cmyk { c, m, y, k }
    }
}
