use std::marker::PhantomData;

use super::convert::ConvertFrom;
use super::{BigEndian, Color, Endianness, LittleEndian, NativeEndian};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Rgb48<E> {
    r: u16,
    g: u16,
    b: u16,
    _endianness: PhantomData<E>,
}

impl<E1: Endianness> Rgb48<E1> {
    pub const fn cast<E2: Endianness>(self) -> Rgb48<E2> {
        let Rgb48 { r, g, b, .. } = self;
        Rgb48 {
            r,
            g,
            b,
            _endianness: PhantomData,
        }
    }
}

pub type Rgb48Ne = Rgb48<NativeEndian>;

pub type Rgb48Be = Rgb48<BigEndian>;

pub type Rgb48Le = Rgb48<LittleEndian>;

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

impl Rgb48<NativeEndian> {
    pub const fn ne(r: u16, b: u16, g: u16) -> Self {
        Self {
            r,
            g,
            b,
            _endianness: PhantomData,
        }
    }

    impl_channel_fn_ne!(r);
    impl_channel_fn_ne!(g);
    impl_channel_fn_ne!(b);

    impl_channel_fn_set_ne!(r, set_r);
    impl_channel_fn_set_ne!(g, set_g);
    impl_channel_fn_set_ne!(b, set_b);
}

impl Rgb48<LittleEndian> {
    pub const fn le(r: u16, b: u16, g: u16) -> Self {
        Self {
            r: r.to_le(),
            g: g.to_le(),
            b: b.to_le(),
            _endianness: PhantomData,
        }
    }

    impl_channel_fn_le!(r);
    impl_channel_fn_le!(g);
    impl_channel_fn_le!(b);

    impl_channel_fn_set_le!(r, set_r);
    impl_channel_fn_set_le!(g, set_g);
    impl_channel_fn_set_le!(b, set_b);
}

impl Rgb48<BigEndian> {
    pub const fn be(r: u16, b: u16, g: u16) -> Self {
        Self {
            r: r.to_be(),
            g: g.to_be(),
            b: b.to_be(),
            _endianness: PhantomData,
        }
    }

    impl_channel_fn_be!(r);
    impl_channel_fn_be!(g);
    impl_channel_fn_be!(b);

    impl_channel_fn_set_be!(r, set_r);
    impl_channel_fn_set_be!(g, set_g);
    impl_channel_fn_set_be!(b, set_b);
}

impl<E> Rgb48<E> {
    pub fn get_channels(self) -> (u16, u16, u16) {
        let Rgb48 { r, g, b, .. } = self;
        (r, g, b)
    }
}

// -------------------------------------------------------------------------- //

#[inline]
fn rgb48_to_rgba(r: u16, g: u16, b: u16) -> (u32, u32, u32, u32) {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    let a = 0xffff;

    (r, g, b, a)
}

impl Color for Rgb48<NativeEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        rgb48_to_rgba(self.r, self.g, self.b)
    }
}

impl Color for Rgb48<BigEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        #[cfg(target_endian = "little")]
        {
            let r = self.r.swap_bytes();
            let g = self.g.swap_bytes();
            let b = self.b.swap_bytes();

            rgb48_to_rgba(r, g, b)
        }

        #[cfg(target_endian = "big")]
        {
            let r = self.r;
            let g = self.g;
            let b = self.b;

            rgb48_to_rgba(r, g, b)
        }
    }
}

impl Color for Rgb48<LittleEndian> {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        #[cfg(target_endian = "little")]
        {
            let r = self.r;
            let g = self.g;
            let b = self.b;

            rgb48_to_rgba(r, g, b)
        }

        #[cfg(target_endian = "big")]
        {
            let r = self.r.swap_bytes();
            let g = self.g.swap_bytes();
            let b = self.b.swap_bytes();

            rgb48_to_rgba(r, g, b)
        }
    }
}

// -------------------------------------------------------------------------- //

impl<C: Color> ConvertFrom<C> for Rgb48<NativeEndian> {
    fn convert_from(c: C) -> Self {
        let (r, g, b, _) = c.as_rgba();
        Rgb48 {
            r: (r & 0xffff) as u16,
            g: (g & 0xffff) as u16,
            b: (b & 0xffff) as u16,
            _endianness: PhantomData,
        }
    }
}

impl<C: Color> ConvertFrom<C> for Rgb48<BigEndian> {
    fn convert_from(c: C) -> Self {
        let (r, g, b, _) = c.as_rgba();
        Rgb48 {
            r: ((r & 0xffff) as u16).to_be(),
            g: ((g & 0xffff) as u16).to_be(),
            b: ((b & 0xffff) as u16).to_be(),
            _endianness: PhantomData,
        }
    }
}

impl<C: Color> ConvertFrom<C> for Rgb48<LittleEndian> {
    fn convert_from(c: C) -> Self {
        let (r, g, b, _) = c.as_rgba();
        Rgb48 {
            r: ((r & 0xffff) as u16).to_le(),
            g: ((g & 0xffff) as u16).to_le(),
            b: ((b & 0xffff) as u16).to_le(),
            _endianness: PhantomData,
        }
    }
}