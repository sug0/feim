use std::io::{self, Read, Write, BufRead};
use std::default::Default;

use crate::image::Format;

pub trait Encode<B> {
    fn encode<W: Write>(&self, w: W, buf: &B) -> io::Result<()>;
}

pub trait Decode<B> {
    fn decode<R: Read>(r: R, opt: DecodeOptions) -> io::Result<B>;
}

pub struct DecodeOptions {
    pub check_header: bool,
}

impl Default for DecodeOptions {
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
