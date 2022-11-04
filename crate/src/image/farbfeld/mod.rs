pub mod stream;

use std::io::{self, Read, Write};

use super::{Dimensions, Format, Image};
use crate::buffer::RawPixBuf;
use crate::color::convert::ConvertInto;
use crate::color::{BigEndian, NativeEndian, Nrgba64};
use crate::impl_format;
use crate::serialize::{Decode, DecodeOptions, Encode, EncodeOptions};
use crate::specialized;

pub struct FarbfeldDecodeOptions {
    pub check_header: bool,
}

impl Default for FarbfeldDecodeOptions {
    fn default() -> Self {
        Self { check_header: true }
    }
}

pub struct Farbfeld;

impl_format! {
    name: Farbfeld,
    id: "feim:ff",
    magic: b"farbfeld????????",
}

impl EncodeOptions for Farbfeld {
    type Options = ();
}

impl DecodeOptions for Farbfeld {
    type Options = FarbfeldDecodeOptions;
}

impl Encode<RawPixBuf<Nrgba64<BigEndian>>> for Farbfeld {
    fn encode<W: Write>(
        mut w: W,
        _opts: (),
        buf: &RawPixBuf<Nrgba64<BigEndian>>,
    ) -> io::Result<()> {
        let width = (buf.width() as u32).to_be_bytes();
        let height = (buf.height() as u32).to_be_bytes();
        let magic = Farbfeld.magic();
        w.write_all(&magic[..8])?;
        w.write_all(&width[..])?;
        w.write_all(&height[..])?;
        w.write_all(buf.as_ref())?;
        Ok(())
    }
}

impl<I: Image + Dimensions> Encode<I, specialized::No> for Farbfeld {
    fn encode<W: Write>(mut w: W, _opts: (), buf: &I) -> io::Result<()> {
        let (width, height) = buf.dimensions();
        {
            let width_ = (width as u32).to_be_bytes();
            let height_ = (height as u32).to_be_bytes();
            let magic = Farbfeld.magic();
            w.write_all(&magic[..8])?;
            w.write_all(&width_[..])?;
            w.write_all(&height_[..])?;
        }
        for y in 0..height {
            for x in 0..width {
                let c = buf.color_get(x, y);
                let c: Nrgba64<BigEndian> = c.convert_into();
                let c: Nrgba64<NativeEndian> = c.cast();
                let c: u64 = c.into();
                let c = c.to_ne_bytes();
                w.write_all(&c[..])?;
            }
        }
        Ok(())
    }
}

impl Decode<RawPixBuf<Nrgba64<BigEndian>>> for Farbfeld {
    fn decode<R: Read>(mut r: R, opt: Self::Options) -> io::Result<RawPixBuf<Nrgba64<BigEndian>>> {
        let mut m: [u8; 16] = [0; 16];
        r.read_exact(&mut m[..])?;
        if opt.check_header && !Farbfeld.is_valid_magic(&m[..]) {
            let k = std::io::ErrorKind::Other;
            let e = std::io::Error::new(k, "Invalid farbfeld magic.");
            return Err(e);
        }
        let width = u32::from_be_bytes([m[8], m[9], m[10], m[11]]) as usize;
        let height = u32::from_be_bytes([m[12], m[13], m[14], m[15]]) as usize;
        let mut buf = RawPixBuf::new(width, height);
        r.read_exact(buf.as_mut())?;
        Ok(buf)
    }
}
