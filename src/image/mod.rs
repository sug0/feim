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
    /// Should return a format id, such as:
    ///
    /// - feim:ff
    /// - feim:png
    /// - feim:jpeg
    fn id(&self) -> &'static str;

    /// Returns the magic string situated at the start of the image file.
    fn magic(&self) -> &'static [u8];

    /// Compares the format's magic string against another byte string.
    fn is_valid_magic(&self, magic: &[u8]) -> bool {
        if magic.len() < self.magic().len() {
            return false
        }
        self.magic().iter()
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
