use crate::color::convert::ConvertInto;
use crate::color::Color;
use crate::image::{Dimensions, Image, ImageMut};

pub trait AsTyped {
    type Pixel: Color;

    fn as_typed(&self) -> &[Self::Pixel];
}

pub trait AsTypedMut {
    type Pixel: Color;

    fn as_typed_mut(&mut self) -> &mut [Self::Pixel];
}

#[derive(Clone, Debug)]
pub struct RawPixBuf<T> {
    width: usize,
    height: usize,
    buf: Box<[T]>,
}

impl<T> RawPixBuf<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let buf = unsafe {
            let elems = width * height;
            let size = elems * std::mem::size_of::<T>();
            let buf = vec![0; size].into_boxed_slice();
            let ptr: *mut T = Box::into_raw(buf) as _;
            Box::from_raw(std::slice::from_raw_parts_mut(ptr, elems))
        };
        RawPixBuf { width, height, buf }
    }

    #[inline]
    pub fn from_vec(width: usize, height: usize, buf: Vec<u8>) -> Result<Self, Vec<u8>> {
        let slice = buf.into_boxed_slice();
        Self::from_slice(width, height, slice).map_err(Vec::from)
    }

    pub fn from_slice(width: usize, height: usize, buf: Box<[u8]>) -> Result<Self, Box<[u8]>> {
        let expected_len = width * height * std::mem::size_of::<T>();
        if buf.len() != expected_len {
            return Err(buf);
        }
        let buf = unsafe { std::mem::transmute(buf) };
        Ok(RawPixBuf { width, height, buf })
    }
}

impl<C: Color> AsTyped for RawPixBuf<C> {
    type Pixel = C;

    fn as_typed(&self) -> &[C] {
        self.buf.as_ref()
    }
}

impl<C: Color> AsTypedMut for RawPixBuf<C> {
    type Pixel = C;

    fn as_typed_mut(&mut self) -> &mut [C] {
        self.buf.as_mut()
    }
}

impl<T> AsRef<[u8]> for RawPixBuf<T> {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            let len = self.buf.len() * std::mem::size_of::<T>();
            let ptr: *const u8 = self.buf.as_ptr() as _;
            std::slice::from_raw_parts(ptr, len)
        }
    }
}

impl<T> AsMut<[u8]> for RawPixBuf<T> {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            let len = self.buf.len() * std::mem::size_of::<T>();
            let ptr: *mut u8 = self.buf.as_mut_ptr() as _;
            std::slice::from_raw_parts_mut(ptr, len)
        }
    }
}

impl<T> Dimensions for RawPixBuf<T> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<C: Copy + Color> Image for RawPixBuf<C> {
    type Pixel = C;

    default fn color_get(&self, x: usize, y: usize) -> C {
        let width = self.width();
        let buffer = self.as_typed();
        buffer[y * width + x]
    }
}

impl<C: Color> ImageMut for RawPixBuf<C> {
    type Pixel = C;

    default fn color_set<P>(&mut self, x: usize, y: usize, color: P)
    where
        P: ConvertInto<C>,
    {
        let width = self.width();
        let buffer = self.as_typed_mut();
        let color: C = color.convert_into();
        buffer[y * width + x] = color;
    }
}
