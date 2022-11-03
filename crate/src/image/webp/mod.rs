mod webp_buf;

use std::io::{self, Read, Write};

use either::Either;
pub use webp::WebPConfig as WebpEncodeOptions;

pub use self::webp_buf::*;
use crate::buffer::RawPixBuf;
use crate::color::{Nrgba, Rgb};
use crate::image::Dimensions;
use crate::impl_format;
use crate::serialize::{Decode, DecodeOptions, Encode, EncodeOptions};

pub struct Webp;

impl_format! {
    name: Webp,
    id: "feim:webp",
    magic: b"RIFF????WEBPVP8",
}

impl DecodeOptions for Webp {
    type Options = ();
}

impl Decode<Either<RgbWebpBuf, NrgbaWebpBuf>> for Webp {
    fn decode<R: Read>(mut r: R, _opt: ()) -> io::Result<Either<RgbWebpBuf, NrgbaWebpBuf>> {
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;
        webp::Decoder::new(&buf[..])
            .decode()
            .map(WebpFeimExt::into_feim)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Webp decode failed"))
    }
}

pub fn simple_config(lossless: bool, quality: f32) -> WebpEncodeOptions {
    let mut opts = WebpEncodeOptions::new().unwrap();
    opts.lossless = if lossless { 1 } else { 0 };
    opts.alpha_compression = if lossless { 0 } else { 1 };
    opts.quality = quality.clamp(0.0, 100.0);
    opts
}

#[inline]
pub fn default_config() -> WebpEncodeOptions {
    simple_config(true, 0.75)
}

impl EncodeOptions for Webp {
    type Options = WebpEncodeOptions;
}

macro_rules! impl_encode {
    ($type:ty, $pixel_layout:expr) => {
        impl Encode<$type> for Webp {
            fn encode<W: Write>(
                mut w: W,
                opts: WebpEncodeOptions,
                image: &$type,
            ) -> io::Result<()> {
                let buf = image.as_ref();
                let (width, height) = image.dimensions();
                let encoded = webp::Encoder::new(buf, $pixel_layout, width as u32, height as u32)
                    .encode_advanced(&opts)
                    .map_err(|e| {
                        io::Error::new(io::ErrorKind::Other, format!("Webp encoding error: {e:?}"))
                    })?;
                w.write_all(&encoded)
            }
        }
    };
}

impl_encode!(RawPixBuf<Rgb>, webp::PixelLayout::Rgb);
impl_encode!(RgbWebpBuf, webp::PixelLayout::Rgb);
impl_encode!(RawPixBuf<Nrgba>, webp::PixelLayout::Rgba);
impl_encode!(NrgbaWebpBuf, webp::PixelLayout::Rgba);
