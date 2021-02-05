use crate::color::{Color, Nrgba};
use crate::image::{Image, ImageMut, Dimensions};

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

    pub fn as_typed(&self) -> &[T] {
        self.buf.as_ref()
    }

    pub fn as_typed_mut(&mut self) -> &mut [T] {
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

impl Image for RawPixBuf<Nrgba> {
    type Pixel = Nrgba;

    fn color_get(&self, x: usize, y: usize) -> Self::Pixel {
        let width = self.width();
        let buffer = self.as_typed();
        buffer[y*width + x]
    }
}

impl ImageMut for RawPixBuf<Nrgba> {
    fn color_set<C: Color>(&mut self, x: usize, y: usize, color: C) {
        let width = self.width();
        let buffer = self.as_typed_mut();
        let color: Nrgba = (&color).into();
        buffer[y*width + x] = color;
    }
}
