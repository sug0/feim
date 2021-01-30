use std::marker::PhantomData;
use std::io::{self, Read, Write, BufReader, BufWriter};

trait Format {
    const NAME: &'static str;
    const MAGIC: &'static [u8];

    fn magic_eq(magic: &[u8]) -> bool {
        if Self::MAGIC.len() != magic.len() {
            return false
        }
        Self::MAGIC.iter()
            .copied()
            .zip(magic.iter().copied())
            .all(|(m, d)| m == d || m == '?' as u8)
    }
}

trait Codec<B: PixelBuffer>: Format {
    fn encode<W: Write>(&self, w: W, buf: &B) -> io::Result<()>;
    fn decode<R: Read>(r: R) -> io::Result<B>;
}

trait PixelBuffer: AsRef<[u8]> + AsMut<[u8]> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

//trait Image: PixelBuffer {
//    fn set ???
//    fn at ???
//}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let stdin_reader = BufReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let image: RawPixBuf<Nrgb64Pix> = Farbfeld::decode(stdin_reader)?;
    write!(&mut stdout_writer, "{:#?}", image).unwrap_or(());
    Ok(())
}

// -------------------------------------------------------------------

struct Farbfeld;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Nrgb64Pix {
    r: u16,
    g: u16,
    b: u16,
    a: u16,
}

#[derive(Clone, Debug)]
struct RawPixBuf<T> {
    width: usize,
    height: usize,
    buf: Box<[u8]>,
    _phantom: PhantomData<T>,
}

impl<T> RawPixBuf<T> {
    fn new(width: usize, height: usize) -> Self {
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

impl Format for Farbfeld {
    const NAME: &'static str = "farbfeld";
    const MAGIC: &'static [u8] = b"farbfeld????????";
}

impl Codec<RawPixBuf<Nrgb64Pix>> for Farbfeld {
    fn encode<W: Write>(&self, mut w: W, buf: &RawPixBuf<Nrgb64Pix>) -> io::Result<()> {
        let width = buf.width.to_be_bytes();
        let height = buf.height.to_be_bytes();
        w.write_all(&Farbfeld::MAGIC[..8])?;
        w.write_all(&width[..])?;
        w.write_all(&height[..])?;
        w.write_all(buf.as_ref())?;
        Ok(())
    }

    fn decode<R: Read>(mut r: R) -> io::Result<RawPixBuf<Nrgb64Pix>> {
        let mut m: [u8; 16] = [0; 16];
        r.read_exact(&mut m[..])?;
        let width = u32::from_be_bytes([m[8], m[9], m[10], m[11]]) as usize;
        let height = u32::from_be_bytes([m[12], m[13], m[14], m[15]]) as usize;
        let mut buf = RawPixBuf::new(width, height);
        r.read_exact(buf.as_mut())?;
        Ok(buf)
    }
}
