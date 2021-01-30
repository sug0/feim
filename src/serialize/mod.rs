use std::io::{self, BufRead};

use crate::image::Format;

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
