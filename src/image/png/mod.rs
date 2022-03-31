use std::default::Default;
use std::io::{self, Write};

use png::{
    Encoder,
    BitDepth,
    ColorType,
    EncodingError,
};

// re-export this stuff
pub use png::{
    FilterType,
    Compression,
};

use crate::image::Dimensions;
use crate::buffer::RawPixBuf;
use crate::impl_format;
use crate::serialize::{
    Encode,
    EncodeOptions,
};
use crate::color::{
    Gray,
    Gray16Ne,
    Nrgba64Ne,
    Nrgba,
    Rgb,
};

pub struct Png;

impl_format! {
    name: Png,
    id: "feim:png",
    magic: b"\x89PNG\r\n\x1a\n",
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

                let mut writer = encoder
                    .write_header()
                    .map_err(|e| {
                        match e {
                            EncodingError::IoError(e) => e,
                            other => io::Error::new(io::ErrorKind::Other, other),
                        }
                    })?;

                writer
                    .write_image_data(buf.as_ref())
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

impl_encode!(Rgb, BitDepth::Eight, ColorType::Rgb);
impl_encode!(Gray, BitDepth::Eight, ColorType::Grayscale);
impl_encode!(Gray16Ne, BitDepth::Sixteen, ColorType::Grayscale);
impl_encode!(Nrgba, BitDepth::Eight, ColorType::Rgba);
impl_encode!(Nrgba64Ne, BitDepth::Sixteen, ColorType::Rgba);

// TODO: default encode / decode
