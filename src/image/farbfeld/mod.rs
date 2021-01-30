use std::io::{self, Read, Write};

use crate::buffer::{PixelBuffer, RawPixBuf};
use super::{Format, Codec, DecodeOptions};
use crate::color::Nrgba64;

pub struct Farbfeld;

impl Format for Farbfeld {
    fn id(&self) -> &'static str { "feim:ff" }
    fn magic(&self) -> &'static [u8] { b"farbfeld????????" }
}

impl Codec<RawPixBuf<Nrgba64>> for Farbfeld {
    fn encode<W: Write>(&self, mut w: W, buf: &RawPixBuf<Nrgba64>) -> io::Result<()> {
        let width = (buf.width() as u32).to_be_bytes();
        let height = (buf.height() as u32).to_be_bytes();
        let magic = Farbfeld.magic();
        w.write_all(&magic[..8])?;
        w.write_all(&width[..])?;
        w.write_all(&height[..])?;
        w.write_all(buf.as_ref())?;
        Ok(())
    }

    fn decode<R: Read>(mut r: R, opt: DecodeOptions) -> io::Result<RawPixBuf<Nrgba64>> {
        let mut m: [u8; 16] = [0; 16];
        r.read_exact(&mut m[..])?;
        if opt.check_header && !Farbfeld.is_valid_magic(&m[..]) {
            let k = std::io::ErrorKind::Other;
            let e = std::io::Error::new(k, "Invalid farbfeld magic.");
            return Err(e)
        }
        let width = u32::from_be_bytes([m[8], m[9], m[10], m[11]]) as usize;
        let height = u32::from_be_bytes([m[12], m[13], m[14], m[15]]) as usize;
        let mut buf = RawPixBuf::new(width, height);
        r.read_exact(buf.as_mut())?;
        Ok(buf)
    }
}
