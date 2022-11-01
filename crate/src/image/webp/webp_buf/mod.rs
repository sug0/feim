pub use webp::WebPImage as WebpBuf;

use crate::color::convert::ConvertInto;
use crate::color::{Color, Nrgba, Rgb};
use crate::image::{Dimensions, Image, ImageMut};

#[derive(Copy, Clone, Debug)]
pub enum WebpPix {
    Nrgba(Nrgba),
    Rgb(Rgb),
}

impl Color for WebpPix {
    fn as_rgba(&self) -> (u32, u32, u32, u32) {
        match self {
            WebpPix::Nrgba(c) => c.as_rgba(),
            WebpPix::Rgb(c) => c.as_rgba(),
        }
    }
}

impl Dimensions for WebpBuf {
    #[inline]
    fn width(&self) -> usize {
        self.width() as usize
    }

    #[inline]
    fn height(&self) -> usize {
        self.height() as usize
    }
}

impl Image for WebpBuf {
    type Pixel = WebpPix;

    fn color_get(&self, x: usize, y: usize) -> Self::Pixel {
        let stride = if self.is_alpha() { 4 } else { 3 };
        let index = (y * Dimensions::width(self) + x) * stride;

        let r = self[index];
        let g = self[index + 1];
        let b = self[index + 2];

        if stride == 4 {
            let a = self[index + 3];
            WebpPix::Nrgba(Nrgba { r, g, b, a })
        } else {
            WebpPix::Rgb(Rgb { r, g, b })
        }
    }
}

impl ImageMut for WebpBuf {
    type Pixel = WebpPix;

    fn color_set<C, ColorSpecialized>(&mut self, _x: usize, _y: usize, _color: C)
    where
        C: ConvertInto<WebpPix, ColorSpecialized> + Color,
    {
        todo!()
    }
}
