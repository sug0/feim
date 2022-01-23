use std::io::{self, BufWriter};

use feim::buffer::{RawPixBuf, AsTypedMut};
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
        image
    };

    Farbfeld::encode(&mut stdout_writer, (), &image)
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
