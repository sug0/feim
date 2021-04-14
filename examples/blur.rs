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

    let mut accum = (0.0, 0.0, 0.0);

    for ky in -1isize..2 {
        for kx in -1isize..2 {
            let mult = KERN[(kx + 1) as usize][(ky + 1) as usize];
            let x = (x as isize + kx) as usize;
            let y = (y as isize + ky) as usize;
            let (r, g, b) = get_clamped(im, x, y);
            accum.0 = r.mul_add(mult, accum.0);
            accum.1 = g.mul_add(mult, accum.1);
            accum.2 = b.mul_add(mult, accum.2);
        }
    }

    Nrgba64 {
        r: accum.0 as u16,
        g: accum.1 as u16,
        b: accum.2 as u16,
        a: 0xffff,
    }
}

fn get_clamped(im: &RawPixBuf<Nrgba64>, mut x: usize, mut y: usize) -> (f32, f32, f32) {
    if x >= im.width() {
        x = im.width() - 1;
    }
    if y >= im.height() {
        y = im.height() - 1;
    }
    let c = im.as_typed()[im.width()*y + x];
    (c.r as f32, c.g as f32, c.b as f32)
}
