use super::convert::ConvertFrom;
use super::Color;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

        let w = 0xffff - k*0x101;
        
        let r = (0xffff - c*0x101) * w / 0xffff;
        let g = (0xffff - m*0x101) * w / 0xffff;
        let b = (0xffff - y*0x101) * w / 0xffff;
        let a = 0xffff;
        
        (r, g, b, a)
    }
}

impl<C: Color> ConvertFrom<C> for Cmyk {
    default fn convert_from(c: C) -> Cmyk {
        let (r, g, b, _) = c.as_rgba();

        let w = std::cmp::min(std::cmp::min(r, g), b);

        if w == 0 {
            return Cmyk {
                c: 0,
                m: 0,
                y: 0,
                k: 0xff,
            }
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
