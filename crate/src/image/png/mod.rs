mod png_buf;

pub use png_buf::*;

use std::io::{self, Read, Write};

use png::{BitDepth, ColorType, Decoder, DecodingError, Encoder, EncodingError, Transformations};

// re-export this stuff
pub use png::{Compression, FilterType};

use crate::buffer::RawPixBuf;
use crate::color::convert::ConvertInto;
use crate::color::{Gray, Gray16Be, Nrgba, Nrgba64Be, Rgb, Rgb48Be};
use crate::image::{Dimensions, Image, ImageMut};
use crate::impl_format;
use crate::serialize::{Decode, DecodeOptions, Encode, EncodeOptions, EncodeSpecialized};
use crate::specialized;

pub struct Png;

impl_format! {
    name: Png,
    id: "feim:png",
    magic: b"\x89PNG\r\n\x1a\n",
}

impl DecodeOptions for Png {
    type Options = ();
}

impl Decode<PngBuf> for Png {
    fn decode<R: Read>(r: R, _opt: ()) -> io::Result<PngBuf> {
        let mut decoder = Decoder::new(r);
        decoder.set_transformations(Transformations::EXPAND);

        let mut reader = decoder.read_info().map_err(|e| match e {
            DecodingError::IoError(e) => e,
            other => io::Error::new(io::ErrorKind::Other, other),
        })?;

        let width = reader.info().width as usize;
        let height = reader.info().height as usize;

        let mut buffer = match reader.output_color_type() {
            (ColorType::Grayscale, BitDepth::Eight) => PngBuf::Gray(RawPixBuf::new(width, height)),
            (ColorType::Grayscale, BitDepth::Sixteen) => {
                PngBuf::Gray16(RawPixBuf::new(width, height))
            }
            (ColorType::Rgba, BitDepth::Eight) => PngBuf::Nrgba(RawPixBuf::new(width, height)),
            (ColorType::Rgba, BitDepth::Sixteen) => PngBuf::Nrgba64(RawPixBuf::new(width, height)),
            (ColorType::Rgb, BitDepth::Eight) => PngBuf::Rgb(RawPixBuf::new(width, height)),
            (ColorType::Rgb, BitDepth::Sixteen) => PngBuf::Rgb48(RawPixBuf::new(width, height)),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Unsupported color type detected",
                ))
            }
        };

        reader.next_frame(buffer.as_mut()).map_err(|e| match e {
            DecodingError::IoError(e) => e,
            other => io::Error::new(io::ErrorKind::Other, other),
        })?;

        Ok(buffer)
    }
}

pub struct PngEncodeOptions {
    pub filter: FilterType,
    pub compression: Compression,
}

impl Default for PngEncodeOptions {
    fn default() -> Self {
        Self {
            filter: FilterType::NoFilter,
            compression: Compression::Default,
        }
    }
}

impl EncodeOptions for Png {
    type Options = PngEncodeOptions;
}

macro_rules! impl_encode {
    ($type:ty, $depth:expr, $color:expr) => {
        impl Encode<RawPixBuf<$type>> for Png {
            fn encode<W: Write>(
                w: W,
                opts: PngEncodeOptions,
                buf: &RawPixBuf<$type>,
            ) -> io::Result<()> {
                let width = (buf.width() & u32::MAX as usize) as u32;
                let height = (buf.height() & u32::MAX as usize) as u32;

                let mut encoder = Encoder::new(w, width, height);
                encoder.set_color($color);
                encoder.set_depth($depth);
                encoder.set_filter(opts.filter);
                encoder.set_compression(opts.compression);

                let mut writer = encoder.write_header().map_err(|e| match e {
                    EncodingError::IoError(e) => e,
                    other => io::Error::new(io::ErrorKind::Other, other),
                })?;

                writer.write_image_data(buf.as_ref()).map_err(|e| match e {
                    EncodingError::IoError(e) => e,
                    other => io::Error::new(io::ErrorKind::Other, other),
                })
            }
        }
    };
}

impl_encode!(Gray, BitDepth::Eight, ColorType::Grayscale);
impl_encode!(Gray16Be, BitDepth::Sixteen, ColorType::Grayscale);
impl_encode!(Nrgba, BitDepth::Eight, ColorType::Rgba);
impl_encode!(Nrgba64Be, BitDepth::Sixteen, ColorType::Rgba);
impl_encode!(Rgb, BitDepth::Eight, ColorType::Rgb);
impl_encode!(Rgb48Be, BitDepth::Sixteen, ColorType::Rgb);

impl Encode<PngBuf> for Png {
    fn encode<W: Write>(w: W, opts: PngEncodeOptions, buf: &PngBuf) -> io::Result<()> {
        match buf {
            PngBuf::Gray(buf) => Png::encode_specialized(w, opts, buf),
            PngBuf::Gray16(buf) => Png::encode_specialized(w, opts, buf),
            PngBuf::Nrgba(buf) => Png::encode_specialized(w, opts, buf),
            PngBuf::Nrgba64(buf) => Png::encode_specialized(w, opts, buf),
            PngBuf::Rgb(buf) => Png::encode_specialized(w, opts, buf),
            PngBuf::Rgb48(buf) => Png::encode_specialized(w, opts, buf),
        }
    }
}

impl<I: Image + Dimensions> Encode<I, specialized::No> for Png {
    fn encode<W: Write>(w: W, opts: PngEncodeOptions, buf: &I) -> io::Result<()> {
        let (width, height) = buf.dimensions();
        let mut new_buf = RawPixBuf::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let c = buf.color_get(x, y);
                let c: Nrgba64Be = c.convert_into();
                new_buf.pixel_set(x, y, c);
            }
        }
        Png::encode_specialized(w, opts, &new_buf)
    }
}
