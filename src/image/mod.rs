pub mod farbfeld;

use std::io::{self, Read, Write};

use crate::buffer::PixelBuffer;

pub trait Format {
    const NAME: &'static str;
    const MAGIC: &'static [u8];

    fn magic_eq(magic: &[u8]) -> bool {
        if Self::MAGIC.len() != magic.len() {
            return false
        }
        Self::MAGIC.iter()
            .copied()
            .zip(magic.iter().copied())
            .all(|(m, d)| m == d || m == '?' as u8)
    }
}

pub trait Codec<B: PixelBuffer>: Format {
    fn encode<W: Write>(&self, w: W, buf: &B) -> io::Result<()>;
    fn decode<R: Read>(r: R) -> io::Result<B>;
}

//trait Image: PixelBuffer {
//    fn set ???
//    fn at ???
//}

