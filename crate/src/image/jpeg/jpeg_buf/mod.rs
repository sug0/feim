use crate::buffer::RawPixBuf;
use crate::color::convert::ConvertInto;
use crate::color::{Cmyk, Color, Gray, Gray16Ne, Rgb};
use crate::image::{Dimensions, Image, ImageMut};

#[derive(Clone, Debug)]
pub enum JpegBuf {
    Gray(RawPixBuf<Gray>),
    Gray16(RawPixBuf<Gray16Ne>),
    Rgb(RawPixBuf<Rgb>),
    Cmyk(RawPixBuf<Cmyk>),
}

impl Dimensions for JpegBuf {
    fn width(&self) -> usize {
        match self {
            JpegBuf::Gray(buf) => buf.width(),
            JpegBuf::Gray16(buf) => buf.width(),
            JpegBuf::Rgb(buf) => buf.width(),
            JpegBuf::Cmyk(buf) => buf.width(),
        }
    }

    fn height(&self) -> usize {
        match self {
            JpegBuf::Gray(buf) => buf.height(),
            JpegBuf::Gray16(buf) => buf.height(),
            JpegBuf::Rgb(buf) => buf.height(),
            JpegBuf::Cmyk(buf) => buf.height(),
        }
    }
}

impl Image for JpegBuf {
    type Pixel = JpegPix;

    fn color_get(&self, x: usize, y: usize) -> Self::Pixel {
        match self {
            JpegBuf::Gray(buf) => JpegPix::Gray(buf.color_get(x, y)),
            JpegBuf::Gray16(buf) => JpegPix::Gray16(buf.color_get(x, y)),
            JpegBuf::Rgb(buf) => JpegPix::Rgb(buf.color_get(x, y)),
            JpegBuf::Cmyk(buf) => JpegPix::Cmyk(buf.color_get(x, y)),
        }
    }
}

impl ImageMut for JpegBuf {
    type Pixel = JpegPix;

    fn color_set<C>(&mut self, x: usize, y: usize, color: C)
    where
        C: ConvertInto<JpegPix> + Color,
    {
        match self {
            JpegBuf::Gray(buf) => buf.color_set(x, y, color),
            JpegBuf::Gray16(buf) => buf.color_set(x, y, color),
            JpegBuf::Rgb(buf) => buf.color_set(x, y, color),
            JpegBuf::Cmyk(buf) => buf.color_set(x, y, color),
        }
    }
}

impl AsRef<[u8]> for JpegBuf {
    fn as_ref(&self) -> &[u8] {
        match self {
            JpegBuf::Gray(buf) => buf.as_ref(),
            JpegBuf::Gray16(buf) => buf.as_ref(),
            JpegBuf::Rgb(buf) => buf.as_ref(),
            JpegBuf::Cmyk(buf) => buf.as_ref(),
        }
    }
}

impl AsMut<[u8]> for JpegBuf {
    fn as_mut(&mut self) -> &mut [u8] {
        match self {
            JpegBuf::Gray(buf) => buf.as_mut(),
            JpegBuf::Gray16(buf) => buf.as_mut(),
            JpegBuf::Rgb(buf) => buf.as_mut(),
            JpegBuf::Cmyk(buf) => buf.as_mut(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum JpegPix {
    Gray(Gray),
    Gray16(Gray16Ne),
    Rgb(Rgb),
    Cmyk(Cmyk),
}

impl Color for JpegPix {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        match self {
            JpegPix::Gray(c) => c.as_rgba(),
            JpegPix::Gray16(c) => c.as_rgba(),
            JpegPix::Rgb(c) => c.as_rgba(),
            JpegPix::Cmyk(c) => c.as_rgba(),
        }
    }
}
