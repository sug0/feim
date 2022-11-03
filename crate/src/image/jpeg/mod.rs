mod jpeg_buf;

pub use jpeg_buf::*;

use std::io::{self, Read, Write};

use jpeg_decoder::{Decoder, Error, PixelFormat};
use jpeg_encoder::{ColorType, Encoder, EncodingError};

use crate::buffer::RawPixBuf;
use crate::color::convert::ConvertInto;
use crate::color::{Cmyk, Gray, Nrgba, Rgb};
use crate::image::{Dimensions, Image, ImageMut};
use crate::impl_format;
use crate::serialize::{Decode, DecodeOptions, Encode, EncodeOptions, EncodeSpecialized};
use crate::specialized;

pub struct Jpeg;

impl_format! {
    name: Jpeg,
    id: "feim:jpeg",
    magic: b"\xff\xd8\xff",
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

macro_rules! impl_encode {
    ($type:ty, $color:expr) => {
        impl Encode<$type> for Jpeg {
            fn encode<W: Write>(
                w: W,
                JpegEncodeOptions { quality }: JpegEncodeOptions,
                buf: &$type,
            ) -> io::Result<()> {
                let width = (buf.width() & 0xffff) as u16;
                let height = (buf.height() & 0xffff) as u16;
                let encoder = Encoder::new(w, quality);
                let color = $color;
                let buf = buf.as_ref();
                encoder
                    .encode(buf, width, height, color)
                    .map_err(|e| match e {
                        EncodingError::IoError(e) => e,
                        other => io::Error::new(io::ErrorKind::Other, other),
                    })
            }
        }
    };
}

impl_encode!(RawPixBuf<Rgb>, ColorType::Rgb);
impl_encode!(RawPixBuf<Gray>, ColorType::Luma);
impl_encode!(RawPixBuf<Cmyk>, ColorType::Cmyk);
impl_encode!(RawPixBuf<Nrgba>, ColorType::Rgba);

#[cfg(feature = "fmt-webp")]
impl_encode!(crate::image::webp::RgbWebpBuf, ColorType::Rgb);
#[cfg(feature = "fmt-webp")]
impl_encode!(crate::image::webp::NrgbaWebpBuf, ColorType::Rgba);

impl Encode<JpegBuf> for Jpeg {
    fn encode<W: Write>(w: W, opts: JpegEncodeOptions, buf: &JpegBuf) -> io::Result<()> {
        match buf {
            JpegBuf::Gray16(_) => todo!(),
            JpegBuf::Gray(buf) => Jpeg::encode_specialized(w, opts, buf),
            JpegBuf::Rgb(buf) => Jpeg::encode_specialized(w, opts, buf),
            JpegBuf::Cmyk(buf) => Jpeg::encode_specialized(w, opts, buf),
        }
    }
}

impl<I: Image + Dimensions> Encode<I, specialized::No> for Jpeg {
    fn encode<W: Write>(w: W, opts: JpegEncodeOptions, buf: &I) -> io::Result<()> {
        let (width, height) = buf.dimensions();
        let mut new_buf = RawPixBuf::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let c = buf.color_get(x, y);
                let c: Rgb = c.convert_into();
                new_buf.pixel_set(x, y, c);
            }
        }
        Jpeg::encode_specialized(w, opts, &new_buf)
    }
}
