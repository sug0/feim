#[cfg(feature = "fmt-farbfeld")]
pub mod farbfeld;

use crate::color::convert::ConvertInto;
use crate::color::Color;

pub trait Format {
    /// Should return a format id, such as:
    ///
    /// - feim:ff
    /// - feim:png
    /// - feim:jpeg
    fn id(&self) -> &'static str;

    /// Returns the magic string situated at the start of the image file.
    fn magic(&self) -> &'static [u8];

    /// Compares the format's magic string against another byte string.
    fn is_valid_magic(&self, magic: &[u8]) -> bool {
        if magic.len() < self.magic().len() {
            return false
        }
        self.magic().iter()
            .copied()
            .zip(magic.iter().copied())
            .all(|(m, n)| m == n || m == '?' as u8)
    }
}

pub trait Dimensions {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

impl<I: Dimensions> Dimensions for &I {
    fn width(&self) -> usize {
        (**self).width()
    }

    fn height(&self) -> usize {
        (**self).height()
    }
}

pub trait Image {
    type Pixel: Color;

    fn color_get(&self, x: usize, y: usize) -> Self::Pixel;
}

pub trait ImageMut {
    type Pixel: Color;

    fn color_set<C>(&mut self, x: usize, y: usize, color: C)
    where
        C: ConvertInto<Self::Pixel>;
}
