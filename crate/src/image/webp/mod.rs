mod webp_buf;

pub use self::webp_buf::*;
use crate::impl_format;

pub struct Webp;

impl_format! {
    name: Webp,
    id: "feim:webp",
    magic: b"RIFF????WEBPVP8",
}
