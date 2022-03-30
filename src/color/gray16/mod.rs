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
pub struct Gray16<E> {
    pub y: u16,
    // TODO: make this field private, and create `Gray16` constructor
    pub _endianness: PhantomData<E>,
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

impl Color for Gray16<NativeEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        gray16_to_rgba(self.y)
    }
}

impl Color for Gray16<BigEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        #[cfg(target_endian = "little")]
        { gray16_to_rgba(self.y.to_le()) }

        #[cfg(target_endian = "big")]
        { gray16_to_rgba(self.y) }
    }
}

impl Color for Gray16<LittleEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        #[cfg(target_endian = "little")]
        { gray16_to_rgba(self.y) }

        #[cfg(target_endian = "big")]
        { gray16_to_rgba(self.y.to_be()) }
    }
}

// -------------------------------------------------------------------------- //

impl<C: Color> ConvertFrom<C> for Gray16<NativeEndian> {
    default fn convert_from(c: C) -> Self {
        let (r, g, b, _) = c.as_rgba();
        let y = ((19595*r + 38470*g + 7471*b + 0x8000) >> 16) as u16;
        Gray16 { y, _endianness: PhantomData }
    }
}

impl<C: Color> ConvertFrom<C> for Gray16<BigEndian> {
    default fn convert_from(c: C) -> Self {
        let (r, g, b, _) = c.as_rgba();
        let y = (((19595*r + 38470*g + 7471*b + 0x8000) >> 16) as u16).to_be();
        Gray16 { y, _endianness: PhantomData }
    }
}

impl<C: Color> ConvertFrom<C> for Gray16<LittleEndian> {
    default fn convert_from(c: C) -> Self {
        let (r, g, b, _) = c.as_rgba();
        let y = (((19595*r + 38470*g + 7471*b + 0x8000) >> 16) as u16).to_le();
        Gray16 { y, _endianness: PhantomData }
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
        { c.y.to_le() }

        #[cfg(target_endian = "big")]
        { c.y }
    }
}

impl From<Gray16<LittleEndian>> for u16 {
    fn from(c: Gray16<LittleEndian>) -> u16 {
        #[cfg(target_endian = "little")]
        { c.y }

        #[cfg(target_endian = "big")]
        { c.y.to_be() }
    }
}

// -------------------------------------------------------------------------- //

impl<E: Endianness> From<u16> for Gray16<E> {
    fn from(y: u16) -> Self {
        Self { y, _endianness: PhantomData }
    }
}
