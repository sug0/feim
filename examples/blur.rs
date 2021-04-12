use std::io::{self, BufReader, BufWriter};

use feim::buffer::{RawPixBuf, AsTyped, AsTypedMut};
use feim::image::farbfeld::Farbfeld;
use feim::image::Dimensions;
use feim::color::Nrgba64;
use feim::serialize::{
    Encode,
    Decode,
    DecodeOptions,
};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let stdin_reader = BufReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let stdout_writer = BufWriter::new(stdout_lock);

    let opts = DecodeOptions { check_header: false };
    let image: RawPixBuf<Nrgba64> = Farbfeld::decode(stdin_reader, opts)?;
    Farbfeld::encode(stdout_writer, &blur(image))
}

fn blur(orig: RawPixBuf<Nrgba64>) -> RawPixBuf<Nrgba64> {
    let mut img = orig.clone();
    let buf = img.as_typed_mut();

    for y in 0..orig.height() {
        for x in 0..orig.width() {
            buf[y*orig.width() + x] = convolve(&orig, x, y);
        }
    }

    img
}

fn convolve(im: &RawPixBuf<Nrgba64>, x: usize, y: usize) -> Nrgba64 {
    static KERN: [[f32; 3]; 3] = [
        [0.0625, 0.125, 0.0625],
        [0.1250, 0.250, 0.1250],
        [0.0625, 0.125, 0.0625],
    ];

    let mut accum = Nrgba64 { r: 0, g: 0, b: 0, a: 0xffff };

    for ky in -1isize..2 {
        for kx in -1isize..2 {
            let mult = KERN[(kx + 1) as usize][(ky + 1) as usize];
            let x = (x as isize + kx) as usize;
            let y = (y as isize + ky) as usize;
            let pix = get_clamped(im, x, y);
            accum.r += (pix.r as f32 * mult) as u16;
            accum.g += (pix.g as f32 * mult) as u16;
            accum.b += (pix.b as f32 * mult) as u16;
        }
    }

    accum
}

fn get_clamped(im: &RawPixBuf<Nrgba64>, mut x: usize, mut y: usize) -> Nrgba64 {
    const CLAMP: u64 = 0x0000_ffff_ffff_ffff;
    let mut clamp = u64::MAX;

    if x >= im.width() {
        clamp = CLAMP;
        x = im.width() - 1;
    }
    if y >= im.height() {
        clamp = CLAMP;
        y = im.height() - 1;
    }

    Nrgba64::from(u64::from(im.as_typed()[im.width()*y + x]) & clamp)
}
