pub use webp::WebPImage as WebpBuf;

use crate::impl_format;
//use crate::image::{Dimensions, Image, ImageMut};
use crate::image::Dimensions;

pub struct Webp;

impl_format! {
    name: Webp,
    id: "feim:webp",
    magic: b"RIFF????WEBPVP8",
}

impl Dimensions for WebpBuf {
    fn width(&self) -> usize {
        self.width() as usize
    }

    fn height(&self) -> usize {
        self.height() as usize
    }
}
