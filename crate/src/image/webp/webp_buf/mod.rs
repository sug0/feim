use either::*;
use webp::WebPImage;

use crate::buffer::{AsTyped, AsTypedMut, RawPixBuf};
use crate::color::convert::ConvertInto;
use crate::color::{Color, Nrgba, Rgb};
use crate::image::{Dimensions, Image, ImageMut};

pub(super) trait WebpFeimExt {
    fn into_feim(self) -> Either<RgbWebpBuf, NrgbaWebpBuf>;
}

impl WebpFeimExt for WebPImage {
    fn into_feim(self) -> Either<RgbWebpBuf, NrgbaWebpBuf> {
        if self.is_alpha() {
            Right(NrgbaWebpBuf { inner: self })
        } else {
            Left(RgbWebpBuf { inner: self })
        }
    }
}

pub type RgbWebpBuf = WebpBuf<false>;

pub type NrgbaWebpBuf = WebpBuf<true>;

pub struct WebpBuf<const HAS_ALPHA: bool> {
    inner: WebPImage,
}

impl<const HAS_ALPHA: bool> WebpBuf<HAS_ALPHA> {
    pub fn into_inner(self) -> WebPImage {
        self.inner
    }
}

impl<const HAS_ALPHA: bool> Dimensions for WebpBuf<HAS_ALPHA> {
    #[inline]
    fn width(&self) -> usize {
        self.inner.width() as usize
    }

    #[inline]
    fn height(&self) -> usize {
        self.inner.height() as usize
    }
}

impl Image for RgbWebpBuf {
    type Pixel = Rgb;

    fn color_get(&self, x: usize, y: usize) -> Self::Pixel {
        let index = (y * Dimensions::width(self) + x) * 3;

        let r = self.inner[index];
        let g = self.inner[index + 1];
        let b = self.inner[index + 2];

        Rgb { r, g, b }
    }
}

impl Image for NrgbaWebpBuf {
    type Pixel = Nrgba;

    fn color_get(&self, x: usize, y: usize) -> Self::Pixel {
        let index = (y * self.width() + x) * 4;

        let r = self.inner[index];
        let g = self.inner[index + 1];
        let b = self.inner[index + 2];
        let a = self.inner[index + 3];

        Nrgba { r, g, b, a }
    }
}

impl ImageMut for RgbWebpBuf {
    type Pixel = Rgb;

    fn color_set<C, ColorSpecialized>(&mut self, x: usize, y: usize, color: C)
    where
        C: ConvertInto<Rgb, ColorSpecialized> + Color,
    {
        let index = (y * self.width() + x) * 3;
        let color = <_ as ConvertInto<Rgb, ColorSpecialized>>::convert_into(color);

        self.inner[index] = color.r;
        self.inner[index + 1] = color.g;
        self.inner[index + 2] = color.b;
    }
}

impl ImageMut for NrgbaWebpBuf {
    type Pixel = Nrgba;

    fn color_set<C, ColorSpecialized>(&mut self, x: usize, y: usize, color: C)
    where
        C: ConvertInto<Nrgba, ColorSpecialized> + Color,
    {
        let index = (y * Dimensions::width(self) + x) * 4;
        let color = <_ as ConvertInto<Nrgba, ColorSpecialized>>::convert_into(color);

        self.inner[index] = color.r;
        self.inner[index + 1] = color.g;
        self.inner[index + 2] = color.b;
        self.inner[index + 3] = color.a;
    }
}

impl From<&RgbWebpBuf> for RawPixBuf<Rgb> {
    fn from(buf: &RgbWebpBuf) -> Self {
        let mut new_buf = RawPixBuf::new_from_dims(buf);
        new_buf.as_mut().copy_from_slice(&buf.inner);
        new_buf
    }
}

impl From<&NrgbaWebpBuf> for RawPixBuf<Nrgba> {
    fn from(buf: &NrgbaWebpBuf) -> Self {
        let mut new_buf = RawPixBuf::new_from_dims(buf);
        new_buf.as_mut().copy_from_slice(&buf.inner);
        new_buf
    }
}

impl<const HAS_ALPHA: bool> AsRef<[u8]> for WebpBuf<HAS_ALPHA> {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl<const HAS_ALPHA: bool> AsMut<[u8]> for WebpBuf<HAS_ALPHA> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

macro_rules! as_typed {
    ($type:ty, $pixel:ty) => {
        impl AsTyped for $type {
            type Pixel = $pixel;

            fn as_typed(&self) -> &[Self::Pixel] {
                unsafe {
                    let len = self.inner.len() / std::mem::size_of::<Self::Pixel>();
                    let ptr: *const Self::Pixel = self.inner.as_ptr() as _;
                    std::slice::from_raw_parts(ptr, len)
                }
            }
        }
    };
}

macro_rules! as_typed_mut {
    ($type:ty, $pixel:ty) => {
        impl AsTypedMut for $type {
            type Pixel = $pixel;

            fn as_typed_mut(&mut self) -> &mut [Self::Pixel] {
                unsafe {
                    let len = self.inner.len() / std::mem::size_of::<Self::Pixel>();
                    let ptr: *mut Self::Pixel = self.inner.as_mut_ptr() as _;
                    std::slice::from_raw_parts_mut(ptr, len)
                }
            }
        }
    };
}

as_typed!(RgbWebpBuf, Rgb);
as_typed!(NrgbaWebpBuf, Nrgba);

as_typed_mut!(RgbWebpBuf, Rgb);
as_typed_mut!(NrgbaWebpBuf, Nrgba);
