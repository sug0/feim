use std::marker::PhantomData;

use super::convert::ConvertFrom;
use super::{
    Endianness,
    NativeEndian,
    LittleEndian,
    BigEndian,
    Color,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Nrgba64<E> {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
    // TODO: make this field private, and create `Gray16` constructor
    pub _endianness: PhantomData<E>,
}

// -------------------------------------------------------------------------- //

#[inline]
fn nrgba64_to_rgba(r: u16, g: u16, b: u16, a: u16) -> (u32, u32, u32, u32) {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    let a = a as u32;

    let r = (r * a) / 0xffff;
    let g = (g * a) / 0xffff;
    let b = (b * a) / 0xffff;

    (r, g, b, a)
}

impl Color for Nrgba64<NativeEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        nrgba64_to_rgba(self.r, self.g, self.b, self.a)
    }
}

impl Color for Nrgba64<BigEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        #[cfg(target_endian = "little")]
        {
            let r = self.r.to_le();
            let g = self.g.to_le();
            let b = self.b.to_le();
            let a = self.a.to_le();

            nrgba64_to_rgba(r, g, b, a)
        }

        #[cfg(target_endian = "big")]
        {
            let r = self.r;
            let g = self.g;
            let b = self.b;
            let a = self.a;

            nrgba64_to_rgba(r, g, b, a)
        }
    }
}

impl Color for Nrgba64<LittleEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        #[cfg(target_endian = "little")]
        {
            let r = self.r;
            let g = self.g;
            let b = self.b;
            let a = self.a;

            nrgba64_to_rgba(r, g, b, a)
        }

        #[cfg(target_endian = "big")]
        {
            let r = self.r.to_be();
            let g = self.g.to_be();
            let b = self.b.to_be();
            let a = self.a.to_be();

            nrgba64_to_rgba(r, g, b, a)
        }
    }
}

// -------------------------------------------------------------------------- //

impl<C: Color> ConvertFrom<C> for Nrgba64<NativeEndian> {
    default fn convert_from(c: C) -> Self {
        let (r, g, b, a) = c.as_rgba();
        Nrgba64 {
            r: (r & 0xffff) as u16,
            g: (g & 0xffff) as u16,
            b: (b & 0xffff) as u16,
            a: (a & 0xffff) as u16,
        }
    }
}

impl<C: Color> ConvertFrom<C> for Nrgba64<BigEndian> {
    default fn convert_from(c: C) -> Self {
        let (r, g, b, a) = c.as_rgba();
        Nrgba64 {
            r: ((r & 0xffff) as u16).to_be(),
            g: ((g & 0xffff) as u16).to_be(),
            b: ((b & 0xffff) as u16).to_be(),
            a: ((a & 0xffff) as u16).to_be(),
        }
    }
}

impl<C: Color> ConvertFrom<C> for Nrgba64<LittleEndian> {
    default fn convert_from(c: C) -> Self {
        let (r, g, b, a) = c.as_rgba();
        Nrgba64 {
            r: ((r & 0xffff) as u16).to_le(),
            g: ((g & 0xffff) as u16).to_le(),
            b: ((b & 0xffff) as u16).to_le(),
            a: ((a & 0xffff) as u16).to_le(),
        }
    }
}

// -------------------------------------------------------------------------- //

impl From<Nrgba64<NativeEndian>> for u64 {
    fn from(c: Nrgba64) -> u64 {
        let r = (c.r as u64) << (16 * 0);
        let g = (c.g as u64) << (16 * 1);
        let b = (c.b as u64) << (16 * 2);
        let a = (c.a as u64) << (16 * 3);
        r | g | b | a
    }
}

impl From<Nrgba64<BigEndian>> for u64 {
    fn from(c: Nrgba64) -> u64 {
        #[cfg(target_endian = "little")]
        let (r, g, b, a) = {
            let r = ((c.r as u64) << (16 * 0)).to_le();
            let g = ((c.g as u64) << (16 * 1)).to_le();
            let b = ((c.b as u64) << (16 * 2)).to_le();
            let a = ((c.a as u64) << (16 * 3)).to_le();
            (r, g, b, a)
        };

        #[cfg(target_endian = "big")]
        let (r, g, b, a) = {
            let r = (c.r as u64) << (16 * 0);
            let g = (c.g as u64) << (16 * 1);
            let b = (c.b as u64) << (16 * 2);
            let a = (c.a as u64) << (16 * 3);
            (r, g, b, a)
        };

        r | g | b | a
    }
}

impl From<Nrgba64<LittleEndian>> for u64 {
    fn from(c: Nrgba64) -> u64 {
        #[cfg(target_endian = "little")]
        let (r, g, b, a) = {
            let r = (c.r as u64) << (16 * 0);
            let g = (c.g as u64) << (16 * 1);
            let b = (c.b as u64) << (16 * 2);
            let a = (c.a as u64) << (16 * 3);
            (r, g, b, a)
        };

        #[cfg(target_endian = "big")]
        let (r, g, b, a) = {
            let r = ((c.r as u64) << (16 * 0)).to_be();
            let g = ((c.g as u64) << (16 * 1)).to_be();
            let b = ((c.b as u64) << (16 * 2)).to_be();
            let a = ((c.a as u64) << (16 * 3)).to_be();
            (r, g, b, a)
        };

        r | g | b | a
    }
}

// -------------------------------------------------------------------------- //

impl<E: Endianness> From<u64> for Nrgba64<E> {
    fn from(c: u64) -> Self {
        let r = (c & 0xffff) as u16;
        let g = ((c & 0xffff0000) >> 16) as u16;
        let b = ((c & 0xffff00000000) >> 32) as u16;
        let a = ((c & 0xffff000000000000) >> 48) as u16;
        Self { r, g, b, a, _endianness: PhantomData }
    }
}
