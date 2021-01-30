pub mod farbfeld;

use std::io::{self, Read, Write};
use std::default::Default;

use crate::buffer::PixelBuffer;

pub struct DecodeOptions {
    pub check_header: bool,
}

impl Default for DecodeOptions {
    fn default() -> Self {
        Self { check_header: true }
    }
}

pub trait Format {
    const NAME: &'static str;
    const MAGIC: &'static [u8];

    fn has_valid_magic(magic: &[u8]) -> bool {
        if Self::MAGIC.len() != magic.len() {
            return false
        }
        Self::MAGIC.iter()
            .copied()
            .zip(magic.iter().copied())
            .all(|(m, n)| m == n || m == '?' as u8)
    }
}

pub trait Codec<B: PixelBuffer>: Format {
    fn encode<W: Write>(&self, w: W, buf: &B) -> io::Result<()>;
    fn decode<R: Read>(r: R, opt: DecodeOptions) -> io::Result<B>;
}

//trait Image: PixelBuffer {
//    fn set ???
//    fn at ???
//}
