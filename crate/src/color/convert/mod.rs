use super::{Color, Endianness, Gray16, Nrgba64, Rgb48};
use crate::buffer::{AsTypedMut, RawPixBuf};
use crate::image::Dimensions;
use crate::specialized::{self, No};

pub trait ConvertFrom<C, Specialized = No>
where
    C: Color,
{
    /// Returns the result of converting a color from `C` into
    /// `Self`.
    fn convert_from(c: C) -> Self;
}

impl<C: Color> ConvertFrom<C, specialized::For<C>> for C {
    #[inline(always)]
    fn convert_from(color: C) -> C {
        color
    }
}

pub trait ConvertInto<C, Specialized = No>
where
    C: Color,
{
    /// Returns the result of converting a color from `Self` into
    /// `C`.
    fn convert_into(self) -> C;
}

impl<A, B, Specialized> ConvertInto<B, Specialized> for A
where
    A: Color,
    B: ConvertFrom<A, Specialized> + Color,
{
    fn convert_into(self) -> B {
        B::convert_from(self)
    }
}

macro_rules! encode_as_impl {
    ($type:ident, $intermediate:ty) => {
        impl<E1> RawPixBuf<$type<E1>> {
            // TODO: change buffer in place, so we don't
            // need to allocate a new one for the conversion
            fn encode_as_impl<E2>(self) -> RawPixBuf<$type<E2>>
            where
                E1: Endianness + Copy,
                E2: Endianness + Copy,
                $type<E1>: Color,
                $type<E2>: Color + From<$intermediate>,
                $intermediate: From<$type<E1>>,
            {
                let (width, height) = self.dimensions();
                let mut new_buffer = RawPixBuf::new(width, height);

                for (pix, new_pix) in self
                    .into_pixels()
                    .into_iter()
                    .zip(new_buffer.as_typed_mut().iter_mut())
                {
                    *new_pix = pix.convert_into();
                }

                new_buffer
            }
        }
    };
}

encode_as_impl!(Nrgba64, u64);
encode_as_impl!(Gray16, u16);
encode_as_impl!(Rgb48, u64);

impl<E1> RawPixBuf<Nrgba64<E1>> {
    #[inline]
    pub fn encode_as<E2>(self) -> RawPixBuf<Nrgba64<E2>>
    where
        E1: Endianness + Copy,
        E2: Endianness + Copy,
        Nrgba64<E1>: Color,
        Nrgba64<E2>: Color + From<u64>,
        u64: From<Nrgba64<E1>>,
    {
        self.encode_as_impl()
    }
}

impl<E1> RawPixBuf<Gray16<E1>> {
    #[inline]
    pub fn encode_as<E2>(self) -> RawPixBuf<Gray16<E2>>
    where
        E1: Endianness + Copy,
        E2: Endianness + Copy,
        Gray16<E1>: Color,
        Gray16<E2>: Color + From<u16>,
        u16: From<Gray16<E1>>,
    {
        self.encode_as_impl()
    }
}

impl<E1> RawPixBuf<Rgb48<E1>> {
    #[inline]
    pub fn encode_as<E2>(self) -> RawPixBuf<Rgb48<E2>>
    where
        E1: Endianness + Copy,
        E2: Endianness + Copy,
        Rgb48<E1>: Color,
        Rgb48<E2>: Color + From<u64>,
        u64: From<Rgb48<E1>>,
    {
        self.encode_as_impl()
    }
}
