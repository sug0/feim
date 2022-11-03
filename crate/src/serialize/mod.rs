use std::io::{self, BufRead, Read, Write};

use crate::image::Format;
use crate::specialized::{Aye, No};

// -------------------------------------------------------------------------- //

pub trait EncodeOptions {
    type Options;
}

pub trait DecodeOptions {
    type Options;
}

// -------------------------------------------------------------------------- //

pub trait Encode<B, Specialized = Aye>: EncodeOptions {
    fn encode<W: Write>(w: W, opts: Self::Options, buf: &B) -> io::Result<()>;
}

// -------------------------------------------------------------------------- //

pub trait EncodeSpecialized<B>: EncodeOptions {
    fn encode_specialized<W: Write>(w: W, opts: Self::Options, buf: &B) -> io::Result<()>;
}

impl<B, F: Encode<B, Aye>> EncodeSpecialized<B> for F {
    #[inline]
    fn encode_specialized<W: Write>(w: W, opts: Self::Options, buf: &B) -> io::Result<()> {
        Self::encode(w, opts, buf)
    }
}

// -------------------------------------------------------------------------- //

pub trait EncodeGeneric<B>: EncodeOptions {
    fn encode_generic<W: Write>(w: W, opts: Self::Options, buf: &B) -> io::Result<()>;
}

impl<B, F: Encode<B, No>> EncodeGeneric<B> for F {
    #[inline]
    fn encode_generic<W: Write>(w: W, opts: Self::Options, buf: &B) -> io::Result<()> {
        Self::encode(w, opts, buf)
    }
}

// -------------------------------------------------------------------------- //

pub trait Decode<B, Specialized = Aye>: DecodeOptions {
    fn decode<R: Read>(r: R, opt: Self::Options) -> io::Result<B>;
}

// -------------------------------------------------------------------------- //

pub trait DecodeSpecialized<B>: DecodeOptions {
    fn decode_specialized<R: Read>(r: R, opt: Self::Options) -> io::Result<B>;
}

impl<B, F: Decode<B, Aye>> DecodeSpecialized<B> for F {
    #[inline]
    fn decode_specialized<R: Read>(r: R, opt: Self::Options) -> io::Result<B> {
        Self::decode(r, opt)
    }
}

// -------------------------------------------------------------------------- //

pub trait DecodeGeneric<B>: DecodeOptions {
    fn decode_generic<R: Read>(r: R, opt: Self::Options) -> io::Result<B>;
}

impl<B, F: Decode<B, No>> DecodeGeneric<B> for F {
    #[inline]
    fn decode_generic<R: Read>(r: R, opt: Self::Options) -> io::Result<B> {
        Self::decode(r, opt)
    }
}

// -------------------------------------------------------------------------- //

pub fn try_format<'f, I, F, R>(mut r: R, formats: F) -> io::Result<I>
where
    F: IntoIterator<Item = (I, &'f dyn Format)>,
    R: BufRead,
{
    let buf = r.fill_buf()?;
    for (i, fmt) in formats {
        if fmt.is_valid_magic(buf) {
            return Ok(i);
        }
    }
    let k = std::io::ErrorKind::Other;
    let e = std::io::Error::new(k, "No matching magic found.");
    Err(e)
}
