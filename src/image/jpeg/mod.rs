use std::io::{self, Read};

use jpeg_decoder::{Decoder, PixelFormat, Error};

use crate::serialize::{Decode, DecodeOptions};
use super::{Image, ImageMut, Dimensions, Format};
use crate::color::{Color, Gray, Rgb, Cmyk};
use crate::color::convert::ConvertInto;
use crate::buffer::RawPixBuf;

pub struct Jpeg;

impl Format for Jpeg {
    fn id(&self) -> &'static str { "feim:jpeg" }
    fn magic(&self) -> &'static [u8] { b"\xff\xd8\xff" }
}

#[derive(Copy, Clone, Debug)]
pub enum JpegPix {
    Gray(Gray),
    Rgb(Rgb),
    Cmyk(Cmyk),
}

impl Color for JpegPix {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        match self {
            JpegPix::Gray(c) => c.as_rgba(),
            JpegPix::Rgb(c) => c.as_rgba(),
            JpegPix::Cmyk(c) => c.as_rgba(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum JpegBuf {
    Gray(RawPixBuf<Gray>),
    Rgb(RawPixBuf<Rgb>),
    Cmyk(RawPixBuf<Cmyk>),
}

impl Dimensions for JpegBuf {
    fn width(&self) -> usize {
        match self {
            JpegBuf::Gray(buf) => buf.width(),
            JpegBuf::Rgb(buf) => buf.width(),
            JpegBuf::Cmyk(buf) => buf.width(),
        }
    }

    fn height(&self) -> usize {
        match self {
            JpegBuf::Gray(buf) => buf.height(),
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
            JpegBuf::Rgb(buf) => buf.color_set(x, y, color),
            JpegBuf::Cmyk(buf) => buf.color_set(x, y, color),
        }
    }
}

impl Decode<JpegBuf> for Jpeg {
    fn decode<R: Read>(r: R, _opt: DecodeOptions) -> io::Result<JpegBuf> {
        let mut d = Decoder::new(r);
        d.read_info().map_err(|e| match e {
            Error::Io(e) => e,
            e => io::Error::new(io::ErrorKind::Other, e),
        })?;
        let buf = d.decode().map_err(|e| match e {
            Error::Io(e) => e,
            e => io::Error::new(io::ErrorKind::Other, e),
        })?;
        let info = d.info().unwrap();
        let w = info.width as usize;
        let h = info.height as usize;
        Ok(match info.pixel_format {
            PixelFormat::L8 => JpegBuf::Gray(RawPixBuf::from_vec(w, h, buf).unwrap()),
            PixelFormat::RGB24 => JpegBuf::Rgb(RawPixBuf::from_vec(w, h, buf).unwrap()),
            PixelFormat::CMYK32 => JpegBuf::Cmyk(RawPixBuf::from_vec(w, h, buf).unwrap()),
        })
    }
}
