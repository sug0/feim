use crate::buffer::RawPixBuf;
use crate::color::convert::ConvertInto;
use crate::color::{Color, Gray, Gray16Be, Nrgba, Nrgba64Be, Rgb, Rgb48Be};
use crate::image::{Dimensions, Image, ImageMut};

#[derive(Clone, Debug)]
pub enum PngBuf {
    Gray(RawPixBuf<Gray>),
    Gray16(RawPixBuf<Gray16Be>),
    Nrgba(RawPixBuf<Nrgba>),
    Nrgba64(RawPixBuf<Nrgba64Be>),
    Rgb(RawPixBuf<Rgb>),
    Rgb48(RawPixBuf<Rgb48Be>),
}

impl Dimensions for PngBuf {
    fn width(&self) -> usize {
        match self {
            PngBuf::Gray(buf) => buf.width(),
            PngBuf::Gray16(buf) => buf.width(),
            PngBuf::Nrgba(buf) => buf.width(),
            PngBuf::Nrgba64(buf) => buf.width(),
            PngBuf::Rgb(buf) => buf.width(),
            PngBuf::Rgb48(buf) => buf.width(),
        }
    }

    fn height(&self) -> usize {
        match self {
            PngBuf::Gray(buf) => buf.height(),
            PngBuf::Gray16(buf) => buf.height(),
            PngBuf::Nrgba(buf) => buf.height(),
            PngBuf::Nrgba64(buf) => buf.height(),
            PngBuf::Rgb(buf) => buf.height(),
            PngBuf::Rgb48(buf) => buf.height(),
        }
    }
}

impl Image for PngBuf {
    type Pixel = PngPix;

    fn color_get(&self, x: usize, y: usize) -> Self::Pixel {
        match self {
            PngBuf::Gray(buf) => PngPix::Gray(buf.color_get(x, y)),
            PngBuf::Gray16(buf) => PngPix::Gray16(buf.color_get(x, y)),
            PngBuf::Nrgba(buf) => PngPix::Nrgba(buf.color_get(x, y)),
            PngBuf::Nrgba64(buf) => PngPix::Nrgba64(buf.color_get(x, y)),
            PngBuf::Rgb(buf) => PngPix::Rgb(buf.color_get(x, y)),
            PngBuf::Rgb48(buf) => PngPix::Rgb48(buf.color_get(x, y)),
        }
    }
}

impl ImageMut for PngBuf {
    type Pixel = PngPix;

    fn color_set<C, ColorSpecialized>(&mut self, x: usize, y: usize, color: C)
    where
        C: ConvertInto<PngPix, ColorSpecialized> + Color,
    {
        let color = <_ as ConvertInto<PngPix, ColorSpecialized>>::convert_into(color);
        match (self, color) {
            (PngBuf::Gray(buf), PngPix::Gray(c)) => buf.pixel_set(x, y, c),
            (PngBuf::Gray(buf), c) => buf.color_set_generic(x, y, c),

            (PngBuf::Gray16(buf), PngPix::Gray16(c)) => buf.pixel_set(x, y, c),
            (PngBuf::Gray16(buf), c) => buf.color_set_generic(x, y, c),

            (PngBuf::Nrgba(buf), PngPix::Nrgba(c)) => buf.pixel_set(x, y, c),
            (PngBuf::Nrgba(buf), c) => buf.color_set_generic(x, y, c),

            (PngBuf::Nrgba64(buf), PngPix::Nrgba64(c)) => buf.pixel_set(x, y, c),
            (PngBuf::Nrgba64(buf), c) => buf.color_set_generic(x, y, c),

            (PngBuf::Rgb(buf), PngPix::Rgb(c)) => buf.pixel_set(x, y, c),
            (PngBuf::Rgb(buf), c) => buf.color_set_generic(x, y, c),

            (PngBuf::Rgb48(buf), PngPix::Rgb48(c)) => buf.pixel_set(x, y, c),
            (PngBuf::Rgb48(buf), c) => buf.color_set_generic(x, y, c),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PngPix {
    Gray(Gray),
    Gray16(Gray16Be),
    Nrgba(Nrgba),
    Nrgba64(Nrgba64Be),
    Rgb(Rgb),
    Rgb48(Rgb48Be),
}

impl Color for PngPix {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        match self {
            PngPix::Gray(c) => c.as_rgba(),
            PngPix::Gray16(c) => c.as_rgba(),
            PngPix::Nrgba(c) => c.as_rgba(),
            PngPix::Nrgba64(c) => c.as_rgba(),
            PngPix::Rgb(c) => c.as_rgba(),
            PngPix::Rgb48(c) => c.as_rgba(),
        }
    }
}

impl AsRef<[u8]> for PngBuf {
    fn as_ref(&self) -> &[u8] {
        match self {
            PngBuf::Gray(buf) => buf.as_ref(),
            PngBuf::Gray16(buf) => buf.as_ref(),
            PngBuf::Nrgba(buf) => buf.as_ref(),
            PngBuf::Nrgba64(buf) => buf.as_ref(),
            PngBuf::Rgb(buf) => buf.as_ref(),
            PngBuf::Rgb48(buf) => buf.as_ref(),
        }
    }
}

impl AsMut<[u8]> for PngBuf {
    fn as_mut(&mut self) -> &mut [u8] {
        match self {
            PngBuf::Gray(buf) => buf.as_mut(),
            PngBuf::Gray16(buf) => buf.as_mut(),
            PngBuf::Nrgba(buf) => buf.as_mut(),
            PngBuf::Nrgba64(buf) => buf.as_mut(),
            PngBuf::Rgb(buf) => buf.as_mut(),
            PngBuf::Rgb48(buf) => buf.as_mut(),
        }
    }
}
