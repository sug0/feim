use std::default::Default;
use std::io::{self, BufRead, Read, Write};

use crate::image::Format;
use crate::specialized::Aye;

pub trait EncodeOptions {
    type Options;
}

pub trait DecodeOptions {
    type Options;
}

pub trait Encode<B, Specialized = Aye>: EncodeOptions {
    fn encode<W: Write>(w: W, opts: Self::Options, buf: &B) -> io::Result<()>;
}

pub trait Decode<B, Specialized = Aye>: DecodeOptions {
    fn decode<R: Read>(r: R, opt: Self::Options) -> io::Result<B>;
}

pub struct GenericDecodeOptions {
    pub check_header: bool,
}

impl Default for GenericDecodeOptions {
    fn default() -> Self {
        Self { check_header: true }
    }
}

pub fn try_format<R: BufRead>(mut r: R, formats: &[&dyn Format]) -> io::Result<usize> {
    let buf = r.fill_buf()?;
    for (i, fmt) in formats.iter().enumerate() {
        if fmt.is_valid_magic(buf) {
            return Ok(i);
        }
    }
    let k = std::io::ErrorKind::Other;
    let e = std::io::Error::new(k, "No matching magic found.");
    Err(e)
}
