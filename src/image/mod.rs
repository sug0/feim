#[cfg(feature = "fmt-farbfeld")]
pub mod farbfeld;

#[cfg(feature = "fmt-jpeg")]
pub mod jpeg;

#[cfg(feature = "fmt-png")]
pub mod png;

use crate::color::convert::ConvertInto;
use crate::color::Color;

#[macro_export]
macro_rules! impl_format {
    (name: $name:ty, id: $id:expr, magic: $magic:expr $(,)?) => {
        impl $crate::image::Format for $name {
            fn id(&self) -> &'static str { $id }
            fn magic(&self) -> &'static [u8] { $magic }
        }
    }
}

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

    fn dimensions(&self) -> (usize, usize) {
        let w = self.width();
        let h = self.height();
        (w, h)
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
        C: ConvertInto<Self::Pixel> + Color;
}
