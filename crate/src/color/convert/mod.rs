use super::{Color, Endianness, Nrgba64};
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

impl<E1> RawPixBuf<Nrgba64<E1>> {
    // TODO: change buffer in place, so we don't
    // need to allocate a new one for the conversion
    pub fn encode_as<E2>(self) -> RawPixBuf<Nrgba64<E2>>
    where
        E1: Endianness + Copy,
        E2: Endianness + Copy,
        Nrgba64<E1>: Color,
        Nrgba64<E2>: Color + From<u64>,
        u64: From<Nrgba64<E1>>,
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
