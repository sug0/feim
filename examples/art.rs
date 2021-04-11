use std::io::{self, BufWriter};

use feim::buffer::{RawPixBuf, AsTyped, AsTypedMut};
use feim::image::farbfeld::Farbfeld;
use feim::serialize::Encode;
use feim::color::Nrgba64;

const DIM: usize = 2000;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let image = {
        let mut image = RawPixBuf::new(DIM, DIM);
        draw_image(image.as_typed_mut());
        blur(&image)
    };

    Farbfeld::encode(&mut stdout_writer, &image)
}

const fn shade(y: u16) -> Nrgba64 {
    Nrgba64 { r: y, g: y, b: y, a: 0xffff }
}

fn draw_image(buf: &mut [Nrgba64]) {
    for y in 0..DIM {
        let yf = y as f32;
        for x in 0..DIM {
            let xf = x as f32;
            let value = 65535.0 * (xf - std::f32::consts::PI*xf).sin();
            let v = ((value as usize) << (x ^ !y)) ^ (value.cos().mul_add(yf, std::f32::consts::PI * xf.cos()) as usize);
            buf[y*DIM + x] = shade((v & 0xffff) as u16);
        }
    }
}

fn blur(orig: &RawPixBuf<Nrgba64>) -> RawPixBuf<Nrgba64> {
    let mut img = orig.clone();
    let a = orig.as_typed();
    let b = img.as_typed_mut();

    for y in 0..DIM {
        for x in 0..DIM {
            b[y*DIM + x] = convolve(a, x, y);
        }
    }

    img
}

fn get_clamped(im: &[Nrgba64], mut x: usize, mut y: usize) -> Nrgba64 {
    const CLAMP: u64 = 0x0000_ffff_ffff_ffff;
    let mut clamp = u64::MAX;

    if x >= DIM {
        clamp = CLAMP;
        x = DIM - 1;
    }
    if y >= DIM {
        clamp = CLAMP;
        y = DIM - 1;
    }

    Nrgba64::from(u64::from(im[DIM*y + x]) & clamp)
}

fn convolve(im: &[Nrgba64], x: usize, y: usize) -> Nrgba64 {
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
