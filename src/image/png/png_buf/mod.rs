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

    fn color_set<C>(&mut self, x: usize, y: usize, color: C)
    where
        C: ConvertInto<PngPix> + Color,
    {
        match self {
            PngBuf::Gray(buf) => buf.color_set(x, y, color),
            PngBuf::Gray16(buf) => buf.color_set(x, y, color),
            PngPix::Nrgba(c) => buf.color_set(x, y, color),
            PngPix::Nrgba64(c) => buf.color_set(x, y, color),
            PngPix::Rgb(c) => buf.color_set(x, y, color),
            PngPix::Rgb48(c) => buf.color_set(x, y, color),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PngPix {
    Gray(Gray),
    Gray16(Gray16Ne),
    Nrgba(Nrgba),
    Nrgba64(Nrgba64Ne),
    Rgb(Rgb),
    Rgb48(Rgb48Ne),
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
