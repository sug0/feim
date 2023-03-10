use std::marker::PhantomData;

use super::convert::ConvertFrom;
use super::{BigEndian, Color, Endianness, LittleEndian, NativeEndian, Zero};
use crate::specialized;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Gray16<E> {
    y: u16,
    _endianness: PhantomData<E>,
}

impl<E1: Endianness> Gray16<E1> {
    pub const fn cast<E2: Endianness>(self) -> Gray16<E2> {
        let Gray16 { y, .. } = self;
        Gray16 {
            y,
            _endianness: PhantomData,
        }
    }
}

pub type Gray16Ne = Gray16<NativeEndian>;

pub type Gray16Be = Gray16<BigEndian>;

pub type Gray16Le = Gray16<LittleEndian>;

// -------------------------------------------------------------------------- //

impl Gray16<NativeEndian> {
    pub const fn ne(y: u16) -> Self {
        Self {
            y,
            _endianness: PhantomData,
        }
    }

    pub const fn y(self) -> u16 {
        self.y
    }

    pub const fn set_y(mut self, y: u16) -> Self {
        self.y = y;
        self
    }
}

impl Gray16<LittleEndian> {
    pub const fn le(y: u16) -> Self {
        Self {
            y: y.to_le(),
            _endianness: PhantomData,
        }
    }

    pub const fn y(self) -> u16 {
        #[cfg(target_endian = "little")]
        {
            self.y
        }

        #[cfg(target_endian = "big")]
        {
            self.y.swap_bytes()
        }
    }

    pub const fn set_y(mut self, y: u16) -> Self {
        #[cfg(target_endian = "little")]
        {
            self.y = y
        }

        #[cfg(target_endian = "big")]
        {
            self.y = y.swap_bytes()
        }

        self
    }
}

impl Gray16<BigEndian> {
    pub const fn be(y: u16) -> Self {
        Self {
            y: y.to_be(),
            _endianness: PhantomData,
        }
    }

    pub const fn y(self) -> u16 {
        #[cfg(target_endian = "little")]
        {
            self.y.swap_bytes()
        }

        #[cfg(target_endian = "big")]
        {
            self.y
        }
    }

    pub const fn set_y(mut self, y: u16) -> Self {
        #[cfg(target_endian = "little")]
        {
            self.y = y.swap_bytes()
        }

        #[cfg(target_endian = "big")]
        {
            self.y = y
        }

        self
    }
}

// -------------------------------------------------------------------------- //

#[inline]
fn gray16_to_rgba(y: u16) -> (u32, u32, u32, u32) {
    let y = y as u32;

    let r = y;
    let g = y;
    let b = y;
    let a = 0xffff;

    (r, g, b, a)
}

impl<E> Zero for Gray16<E>
where
    Gray16<E>: Color + Copy,
{
    const ZERO: Self = Gray16 {
        y: 0,
        _endianness: PhantomData,
    };
}

impl Color for Gray16<NativeEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        gray16_to_rgba(self.y)
    }
}

impl Color for Gray16<BigEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        #[cfg(target_endian = "little")]
        {
            gray16_to_rgba(self.y.swap_bytes())
        }

        #[cfg(target_endian = "big")]
        {
            gray16_to_rgba(self.y)
        }
    }
}

impl Color for Gray16<LittleEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        #[cfg(target_endian = "little")]
        {
            gray16_to_rgba(self.y)
        }

        #[cfg(target_endian = "big")]
        {
            gray16_to_rgba(self.y.swap_bytes())
        }
    }
}

// -------------------------------------------------------------------------- //

impl<E1, E2> ConvertFrom<Gray16<E1>, specialized::Aye> for Gray16<E2>
where
    E1: Endianness,
    E2: Endianness,
    Gray16<E1>: Color,
    Gray16<E2>: Color + From<u16>,
    u16: From<Gray16<E1>>,
{
    fn convert_from(c: Gray16<E1>) -> Gray16<E2> {
        let c: u16 = c.into();
        c.into()
    }
}

impl<C: Color> ConvertFrom<C> for Gray16<NativeEndian> {
    fn convert_from(c: C) -> Self {
        let (r, g, b, _) = c.as_rgba();
        let y = ((19595 * r + 38470 * g + 7471 * b + 0x8000) >> 16) as u16;
        Gray16 {
            y,
            _endianness: PhantomData,
        }
    }
}

impl<C: Color> ConvertFrom<C> for Gray16<BigEndian> {
    fn convert_from(c: C) -> Self {
        let (r, g, b, _) = c.as_rgba();
        let y = (((19595 * r + 38470 * g + 7471 * b + 0x8000) >> 16) as u16).to_be();
        Gray16 {
            y,
            _endianness: PhantomData,
        }
    }
}

impl<C: Color> ConvertFrom<C> for Gray16<LittleEndian> {
    fn convert_from(c: C) -> Self {
        let (r, g, b, _) = c.as_rgba();
        let y = (((19595 * r + 38470 * g + 7471 * b + 0x8000) >> 16) as u16).to_le();
        Gray16 {
            y,
            _endianness: PhantomData,
        }
    }
}

// -------------------------------------------------------------------------- //

impl From<Gray16<NativeEndian>> for u16 {
    fn from(c: Gray16<NativeEndian>) -> u16 {
        c.y
    }
}

impl From<Gray16<BigEndian>> for u16 {
    fn from(c: Gray16<BigEndian>) -> u16 {
        #[cfg(target_endian = "little")]
        {
            c.y.swap_bytes()
        }

        #[cfg(target_endian = "big")]
        {
            c.y
        }
    }
}

impl From<Gray16<LittleEndian>> for u16 {
    fn from(c: Gray16<LittleEndian>) -> u16 {
        #[cfg(target_endian = "little")]
        {
            c.y
        }

        #[cfg(target_endian = "big")]
        {
            c.y.swap_bytes()
        }
    }
}

// -------------------------------------------------------------------------- //

impl From<u16> for Gray16<NativeEndian> {
    fn from(y: u16) -> Self {
        Self {
            y,
            _endianness: PhantomData,
        }
    }
}

impl From<u16> for Gray16<BigEndian> {
    fn from(y: u16) -> Self {
        Self {
            y: y.to_be(),
            _endianness: PhantomData,
        }
    }
}

impl From<u16> for Gray16<LittleEndian> {
    fn from(y: u16) -> Self {
        Self {
            y: y.to_le(),
            _endianness: PhantomData,
        }
    }
}
