use std::marker::PhantomData;

pub trait PixelBuffer: AsRef<[u8]> + AsMut<[u8]> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

#[derive(Clone, Debug)]
pub struct RawPixBuf<T> {
    width: usize,
    height: usize,
    buf: Box<[u8]>,
    _phantom: PhantomData<T>,
}

impl<T> RawPixBuf<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let _phantom = PhantomData;
        let size = width * height * std::mem::size_of::<T>();
        let buf = vec![0; size].into_boxed_slice();
        RawPixBuf { width, height, buf, _phantom }
    }

    //fn into_raw_parts(self) -> (usize, usize, Box<[T]>) {
    //    let buf = unsafe {
    //        let ptr: *mut [T] = Box::into_raw(self.buf) as _;
    //        Box::from_raw(ptr)
    //    };
    //    (self.width, self.height, buf)
    //}

    //unsafe fn from_raw_parts(width: usize, height: usize, buf: Box<[T]>) -> Self {
    //    let _phantom = PhantomData;
    //    let buf = unsafe {
    //        let ptr: *mut [u8] = Box::into_raw(buf) as _;
    //        Box::from_raw(ptr)
    //    };
    //    RawPixBuf { width, height, buf, _phantom }
    //}
}

impl<T> AsRef<[u8]> for RawPixBuf<T> {
    fn as_ref(&self) -> &[u8] {
        self.buf.as_ref()
    }
}

impl<T> AsMut<[u8]> for RawPixBuf<T> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.buf.as_mut()
    }
}

impl<T> PixelBuffer for RawPixBuf<T> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}
