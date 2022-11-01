use crate::impl_format;

pub struct Webp;

impl_format! {
    name: Webp,
    id: "feim:webp",
    magic: b"RIFF????WEBPVP8",
}
