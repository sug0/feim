use std::marker::PhantomData;

use super::convert::ConvertFrom;
use super::{BigEndian, Color, Endianness, LittleEndian, NativeEndian, Zero};
use crate::specialized;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Nrgba64<E> {
    r: u16,
    g: u16,
    b: u16,
    a: u16,
    _endianness: PhantomData<E>,
}

impl<E1: Endianness> Nrgba64<E1> {
    pub const fn cast<E2: Endianness>(self) -> Nrgba64<E2> {
        let Nrgba64 { r, g, b, a, .. } = self;
        Nrgba64 {
            r,
            g,
            b,
            a,
            _endianness: PhantomData,
        }
    }
}

pub type Nrgba64Ne = Nrgba64<NativeEndian>;

pub type Nrgba64Be = Nrgba64<BigEndian>;

pub type Nrgba64Le = Nrgba64<LittleEndian>;

// -------------------------------------------------------------------------- //

macro_rules! impl_channel_fn_set_ne {
    ($comp:ident, $set_channel:ident) => {
        pub const fn $set_channel(mut self, value: u16) -> Self {
            self.$comp = value;
            self
        }
    };
}

macro_rules! impl_channel_fn_set_le {
    ($comp:ident, $set_channel:ident) => {
        pub const fn $set_channel(mut self, value: u16) -> Self {
            #[cfg(target_endian = "little")]
            {
                self.$comp = value
            }

            #[cfg(target_endian = "big")]
            {
                self.$comp = value.swap_bytes()
            }

            self
        }
    };
}

macro_rules! impl_channel_fn_set_be {
    ($comp:ident, $set_channel:ident) => {
        pub const fn $set_channel(mut self, value: u16) -> Self {
            #[cfg(target_endian = "little")]
            {
                self.$comp = value.swap_bytes()
            }

            #[cfg(target_endian = "big")]
            {
                self.$comp = value
            }

            self
        }
    };
}

macro_rules! impl_channel_fn_ne {
    ($c:ident) => {
        pub const fn $c(self) -> u16 {
            self.$c
        }
    };
}

macro_rules! impl_channel_fn_le {
    ($c:ident) => {
        pub const fn $c(self) -> u16 {
            #[cfg(target_endian = "little")]
            {
                self.$c
            }

            #[cfg(target_endian = "big")]
            {
                self.$c.swap_bytes()
            }
        }
    };
}

macro_rules! impl_channel_fn_be {
    ($c:ident) => {
        pub const fn $c(self) -> u16 {
            #[cfg(target_endian = "little")]
            {
                self.$c.swap_bytes()
            }

            #[cfg(target_endian = "big")]
            {
                self.$c
            }
        }
    };
}

impl<E> Nrgba64<E> {
    pub fn get_channels(self) -> (u16, u16, u16, u16) {
        let Nrgba64 { r, g, b, a, .. } = self;
        (r, g, b, a)
    }
}

impl Nrgba64<NativeEndian> {
    pub const fn ne(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self {
            r,
            g,
            b,
            a,
            _endianness: PhantomData,
        }
    }

    impl_channel_fn_ne!(r);
    impl_channel_fn_ne!(g);
    impl_channel_fn_ne!(b);
    impl_channel_fn_ne!(a);

    impl_channel_fn_set_ne!(r, set_r);
    impl_channel_fn_set_ne!(g, set_g);
    impl_channel_fn_set_ne!(b, set_b);
    impl_channel_fn_set_ne!(a, set_a);
}

impl Nrgba64<LittleEndian> {
    pub const fn le(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self {
            r: r.to_le(),
            g: g.to_le(),
            b: b.to_le(),
            a: a.to_le(),
            _endianness: PhantomData,
        }
    }

    impl_channel_fn_le!(r);
    impl_channel_fn_le!(g);
    impl_channel_fn_le!(b);
    impl_channel_fn_le!(a);

    impl_channel_fn_set_le!(r, set_r);
    impl_channel_fn_set_le!(g, set_g);
    impl_channel_fn_set_le!(b, set_b);
    impl_channel_fn_set_le!(a, set_a);
}

impl Nrgba64<BigEndian> {
    pub const fn be(r: u16, g: u16, b: u16, a: u16) -> Self {
        Self {
            r: r.to_be(),
            g: g.to_be(),
            b: b.to_be(),
            a: a.to_be(),
            _endianness: PhantomData,
        }
    }

    impl_channel_fn_be!(r);
    impl_channel_fn_be!(g);
    impl_channel_fn_be!(b);
    impl_channel_fn_be!(a);

    impl_channel_fn_set_be!(r, set_r);
    impl_channel_fn_set_be!(g, set_g);
    impl_channel_fn_set_be!(b, set_b);
    impl_channel_fn_set_be!(a, set_a);
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

impl<E> Zero for Nrgba64<E>
where
    Nrgba64<E>: Color + Copy,
{
    const ZERO: Self = Nrgba64 {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
        _endianness: PhantomData,
    };
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
            let r = self.r.swap_bytes();
            let g = self.g.swap_bytes();
            let b = self.b.swap_bytes();
            let a = self.a.swap_bytes();

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
            let r = self.r.swap_bytes();
            let g = self.g.swap_bytes();
            let b = self.b.swap_bytes();
            let a = self.a.swap_bytes();

            nrgba64_to_rgba(r, g, b, a)
        }
    }
}

// -------------------------------------------------------------------------- //

impl<E1, E2> ConvertFrom<Nrgba64<E1>, specialized::Aye> for Nrgba64<E2>
where
    E1: Endianness,
    E2: Endianness,
    Nrgba64<E1>: Color,
    Nrgba64<E2>: Color + From<u64>,
    u64: From<Nrgba64<E1>>,
{
    fn convert_from(c: Nrgba64<E1>) -> Nrgba64<E2> {
        let c: u64 = c.into();
        c.into()
    }
}

impl<C: Color> ConvertFrom<C> for Nrgba64<NativeEndian> {
    fn convert_from(c: C) -> Self {
        let (r, g, b, a) = c.as_rgba();
        Nrgba64 {
            r: (r & 0xffff) as u16,
            g: (g & 0xffff) as u16,
            b: (b & 0xffff) as u16,
            a: (a & 0xffff) as u16,
            _endianness: PhantomData,
        }
    }
}

impl<C: Color> ConvertFrom<C> for Nrgba64<BigEndian> {
    fn convert_from(c: C) -> Self {
        let (r, g, b, a) = c.as_rgba();
        Nrgba64 {
            r: ((r & 0xffff) as u16).swap_bytes(),
            g: ((g & 0xffff) as u16).swap_bytes(),
            b: ((b & 0xffff) as u16).swap_bytes(),
            a: ((a & 0xffff) as u16).swap_bytes(),
            _endianness: PhantomData,
        }
    }
}

impl<C: Color> ConvertFrom<C> for Nrgba64<LittleEndian> {
    fn convert_from(c: C) -> Self {
        let (r, g, b, a) = c.as_rgba();
        Nrgba64 {
            r: ((r & 0xffff) as u16).swap_bytes(),
            g: ((g & 0xffff) as u16).swap_bytes(),
            b: ((b & 0xffff) as u16).swap_bytes(),
            a: ((a & 0xffff) as u16).swap_bytes(),
            _endianness: PhantomData,
        }
    }
}

// -------------------------------------------------------------------------- //

impl From<Nrgba64<NativeEndian>> for u64 {
    fn from(c: Nrgba64<NativeEndian>) -> u64 {
        let r = c.r as u64;
        let g = (c.g as u64) << 16;
        let b = (c.b as u64) << (16 * 2);
        let a = (c.a as u64) << (16 * 3);
        r | g | b | a
    }
}

impl From<Nrgba64<BigEndian>> for u64 {
    fn from(c: Nrgba64<BigEndian>) -> u64 {
        #[cfg(target_endian = "little")]
        let (r, g, b, a) = {
            let r = (c.r as u64).swap_bytes();
            let g = ((c.g as u64) << 16).swap_bytes();
            let b = ((c.b as u64) << (16 * 2)).swap_bytes();
            let a = ((c.a as u64) << (16 * 3)).swap_bytes();
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
    fn from(c: Nrgba64<LittleEndian>) -> u64 {
        #[cfg(target_endian = "little")]
        let (r, g, b, a) = {
            let r = c.r as u64;
            let g = (c.g as u64) << 16;
            let b = (c.b as u64) << (16 * 2);
            let a = (c.a as u64) << (16 * 3);
            (r, g, b, a)
        };

        #[cfg(target_endian = "big")]
        let (r, g, b, a) = {
            let r = (c.r as u64).swap_bytes();
            let g = ((c.g as u64) << 16).swap_bytes();
            let b = ((c.b as u64) << (16 * 2)).swap_bytes();
            let a = ((c.a as u64) << (16 * 3)).swap_bytes();
            (r, g, b, a)
        };

        r | g | b | a
    }
}

// -------------------------------------------------------------------------- //

fn get_components(c: u64) -> (u16, u16, u16, u16) {
    let r = (c & 0xffff) as u16;
    let g = ((c & 0xffff0000) >> 16) as u16;
    let b = ((c & 0xffff00000000) >> 32) as u16;
    let a = ((c & 0xffff000000000000) >> 48) as u16;
    (r, g, b, a)
}

impl From<u64> for Nrgba64<NativeEndian> {
    fn from(c: u64) -> Self {
        let (r, g, b, a) = get_components(c);
        Self {
            r,
            g,
            b,
            a,
            _endianness: PhantomData,
        }
    }
}

impl From<u64> for Nrgba64<BigEndian> {
    fn from(c: u64) -> Self {
        let (r, g, b, a) = get_components(c);
        Self {
            r: r.to_be(),
            g: g.to_be(),
            b: b.to_be(),
            a: a.to_be(),
            _endianness: PhantomData,
        }
    }
}

impl From<u64> for Nrgba64<LittleEndian> {
    fn from(c: u64) -> Self {
        let (r, g, b, a) = get_components(c);
        Self {
            r: r.to_le(),
            g: g.to_le(),
            b: b.to_le(),
            a: a.to_le(),
            _endianness: PhantomData,
        }
    }
}
