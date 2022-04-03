use crate::buffer::RawPixBuf;
use crate::color::convert::ConvertInto;
use crate::image::{Image, ImageMut, Dimensions};
use crate::color::{
    Gray,
    Gray16Ne,
    Nrgba,
    Nrgba64Ne,
    Rgb,
    Rgb48Ne,
    Cmyk,
    Color,
};

#[derive(Clone, Debug)]
pub enum PngBuf {
    Gray(RawPixBuf<Gray>),
    Gray16(RawPixBuf<Gray16Ne>),
    Nrgba(RawPixBuf<Nrgba>),
    Nrgba64(RawPixBuf<Nrgba64Ne>),
    Rgb(RawPixBuf<Rgb>),
    Rgb48(RawPixBuf<Rgb48Ne>),
    Cmyk(RawPixBuf<Cmyk>),
}

impl Dimensions for PngBuf {
    fn width(&self) -> usize {
        match self {
            PngBuf::Gray(buf) => buf.width(),
            PngBuf::Gray16(buf) => buf.width(),
            PngBuf::Rgb(buf) => buf.width(),
            PngBuf::Cmyk(buf) => buf.width(),
        }
    }

    fn height(&self) -> usize {
        match self {
            PngBuf::Gray(buf) => buf.height(),
            PngBuf::Gray16(buf) => buf.height(),
            PngBuf::Rgb(buf) => buf.height(),
            PngBuf::Cmyk(buf) => buf.height(),
        }
    }
}

impl Image for PngBuf {
    type Pixel = PngPix;

    fn color_get(&self, x: usize, y: usize) -> Self::Pixel {
        match self {
            PngBuf::Gray(buf) => PngPix::Gray(buf.color_get(x, y)),
            PngBuf::Gray16(buf) => PngPix::Gray16(buf.color_get(x, y)),
            PngBuf::Rgb(buf) => PngPix::Rgb(buf.color_get(x, y)),
            PngBuf::Cmyk(buf) => PngPix::Cmyk(buf.color_get(x, y)),
        }
    }
}

impl ImageMut for PngBuf {
    type Pixel = PngPix;

    fn color_set<C>(&mut self, x: usize, y: usize, color: C)
    where
        C: ConvertInto<PngPix> + Color,
    {
        match self {
            PngBuf::Gray(buf) => buf.color_set(x, y, color),
            PngBuf::Gray16(buf) => buf.color_set(x, y, color),
            PngBuf::Rgb(buf) => buf.color_set(x, y, color),
            PngBuf::Cmyk(buf) => buf.color_set(x, y, color),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PngPix {
    Gray(Gray),
    Gray16(Gray16Ne),
    Rgb(Rgb),
    Cmyk(Cmyk),
}

impl Color for PngPix {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        match self {
            PngPix::Gray(c) => c.as_rgba(),
            PngPix::Gray16(c) => c.as_rgba(),
            PngPix::Rgb(c) => c.as_rgba(),
            PngPix::Cmyk(c) => c.as_rgba(),
        }
    }
}
