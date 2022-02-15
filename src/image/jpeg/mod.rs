use std::io::{self, Read, Write};

use jpeg_decoder::{Decoder, PixelFormat, Error};
use jpeg_encoder::{Encoder, ColorType, EncodingError};

use super::{Image, ImageMut, Dimensions};
use crate::color::convert::ConvertInto;
use crate::buffer::RawPixBuf;
use crate::impl_format;
use crate::serialize::{
    Encode,
    Decode,
    EncodeOptions,
    DecodeOptions,
};
use crate::color::{
    NativeEndian,
    Color,
    Gray,
    Gray16,
    Nrgba,
    Rgb,
    Cmyk,
};

pub struct Jpeg;

impl_format! {
    name: Jpeg,
    id: "feim:jpeg",
    magic: b"\xff\xd8\xff",
}

#[derive(Copy, Clone, Debug)]
pub enum JpegPix {
    Gray(Gray),
    Gray16(Gray16<NativeEndian>),
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

#[derive(Clone, Debug)]
pub enum JpegBuf {
    Gray(RawPixBuf<Gray>),
    Gray16(RawPixBuf<Gray16<NativeEndian>>),
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

pub struct JpegEncodeOptions {
    quality: u8,
}

impl JpegEncodeOptions {
    pub fn new(quality: u8) -> Option<Self> {
        if quality <= 100 {
            Some(Self { quality })
        } else {
            None
        }
    }
}

impl EncodeOptions for Jpeg {
    type Options = JpegEncodeOptions;
}

impl DecodeOptions for Jpeg {
    type Options = ();
}

impl Decode<JpegBuf> for Jpeg {
    fn decode<R: Read>(r: R, _opt: ()) -> io::Result<JpegBuf> {
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
            PixelFormat::L16 => JpegBuf::Gray16(RawPixBuf::from_vec(w, h, buf).unwrap()),
            PixelFormat::RGB24 => JpegBuf::Rgb(RawPixBuf::from_vec(w, h, buf).unwrap()),
            PixelFormat::CMYK32 => JpegBuf::Cmyk(RawPixBuf::from_vec(w, h, buf).unwrap()),
        })
    }
}

macro_rules! impl_encode {
    ($type:ty, $color:expr) => {
        impl Encode<RawPixBuf<$type>> for Jpeg {
            fn encode<W: Write>(
                w: W,
                JpegEncodeOptions { quality }: JpegEncodeOptions,
                buf: &RawPixBuf<$type>,
            ) -> io::Result<()> {
                let width = (buf.width() & 0xffff) as u16;
                let height = (buf.height() & 0xffff) as u16;
                let encoder = Encoder::new(w, quality);
                let color = $color;
                let buf = buf.as_ref();
                encoder
                    .encode(buf, width, height, color)
                    .map_err(|e| {
                        match e {
                            EncodingError::IoError(e) => e,
                            other => io::Error::new(io::ErrorKind::Other, other),
                        }
                    })
            }
        }
    }
}

impl_encode!(Rgb, ColorType::Rgb);
impl_encode!(Gray, ColorType::Luma);
impl_encode!(Cmyk, ColorType::Cmyk);
impl_encode!(Nrgba, ColorType::Rgba);

// TODO: default encode / decode for jpeg
