use std::marker::PhantomData;

use super::convert::ConvertFrom;
use super::{
    NativeEndian,
    LittleEndian,
    BigEndian,
    Color,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Rgb48<E> {
    r: u16,
    g: u16,
    b: u16,
    _endianness: PhantomData<E>,
}

pub type Rgb48Ne = Rgb48<NativeEndian>;

pub type Rgb48Be = Rgb48<BigEndian>;

pub type Rgb48Le = Rgb48<LittleEndian>;

// -------------------------------------------------------------------------- //

macro_rules! impl_constructor {
    ($endianness:ident) => {
        pub const fn $endianness(r: u16, b: u16, g: u16) -> Self {
            Self { r, g, b, _endianness: PhantomData }
        }
    }
}

macro_rules! impl_component_fn_set_ne {
    ($comp:ident, $set_component:ident) => {
        pub const fn $set_component(mut self, value: u16) -> Self {
            self.$comp = value;
            self
        }
    }
}

macro_rules! impl_component_fn_set_le {
    ($comp:ident, $set_component:ident) => {
        pub const fn $set_component(mut self, value: u16) -> Self {
            #[cfg(target_endian = "little")]
            { self.$comp = value }

            #[cfg(target_endian = "big")]
            { self.$comp = value.to_be() }

            self
        }
    }
}

macro_rules! impl_component_fn_set_be {
    ($comp:ident, $set_component:ident) => {
        pub const fn $set_component(mut self, value: u16) -> Self {
            #[cfg(target_endian = "little")]
            { self.$comp = value.to_le() }

            #[cfg(target_endian = "big")]
            { self.$comp = value }

            self
        }
    }
}

macro_rules! impl_component_fn_ne {
    ($c:ident) => {
        pub const fn $c(self) -> u16 {
            self.$c
        }
    }
}

macro_rules! impl_component_fn_le {
    ($c:ident) => {
        pub const fn $c(self) -> u16 {
            #[cfg(target_endian = "little")]
            { self.$c }

            #[cfg(target_endian = "big")]
            { self.$c.to_be() }
        }
    }
}

macro_rules! impl_component_fn_be {
    ($c:ident) => {
        pub const fn $c(self) -> u16 {
            #[cfg(target_endian = "little")]
            { self.$c.to_le() }

            #[cfg(target_endian = "big")]
            { self.$c }
        }
    }
}

impl Rgb48<NativeEndian> {
    impl_constructor!(ne);

    impl_component_fn_ne!(r);
    impl_component_fn_ne!(g);
    impl_component_fn_ne!(b);

    impl_component_fn_set_ne!(r, set_r);
    impl_component_fn_set_ne!(g, set_g);
    impl_component_fn_set_ne!(b, set_b);
}

impl Rgb48<LittleEndian> {
    impl_constructor!(le);

    impl_component_fn_le!(r);
    impl_component_fn_le!(g);
    impl_component_fn_le!(b);

    impl_component_fn_set_le!(r, set_r);
    impl_component_fn_set_le!(g, set_g);
    impl_component_fn_set_le!(b, set_b);
}

impl Rgb48<BigEndian> {
    impl_constructor!(be);

    impl_component_fn_be!(r);
    impl_component_fn_be!(g);
    impl_component_fn_be!(b);

    impl_component_fn_set_be!(r, set_r);
    impl_component_fn_set_be!(g, set_g);
    impl_component_fn_set_be!(b, set_b);
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
            let r = self.r.to_le();
            let g = self.g.to_le();
            let b = self.b.to_le();

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
            let r = self.r.to_be();
            let g = self.g.to_be();
            let b = self.b.to_be();

            rgb48_to_rgba(r, g, b)
        }
    }
}

// -------------------------------------------------------------------------- //

impl<C: Color> ConvertFrom<C> for Rgb48<NativeEndian> {
    default fn convert_from(c: C) -> Self {
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
    default fn convert_from(c: C) -> Self {
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
    default fn convert_from(c: C) -> Self {
        let (r, g, b, _) = c.as_rgba();
        Rgb48 {
            r: ((r & 0xffff) as u16).to_le(),
            g: ((g & 0xffff) as u16).to_le(),
            b: ((b & 0xffff) as u16).to_le(),
            _endianness: PhantomData,
        }
    }
}
