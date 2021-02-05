use std::io::{self, BufWriter};

use feim::image::farbfeld::Farbfeld;
use feim::serialize::Encode;
use feim::buffer::RawPixBuf;
use feim::color::Nrgba;

const DIM: usize = 500;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let mut image = RawPixBuf::new(DIM, DIM);
    draw_image(image.as_typed_mut());

    Farbfeld::encode(&mut stdout_writer, &&image)
}

fn draw_image(buf: &mut [Nrgba]) {
    const HALF: usize = DIM / 2;
    const RED: Nrgba = Nrgba { r: 255, g: 0, b: 0, a: 255 };
    const WHITE: Nrgba = Nrgba { r: 255, g: 255, b: 255, a: 255 };

    let mut color = WHITE;
    const DIST: u8 = 128;

    for y in 0..DIM {
        let yh = y - HALF;
        for x in 0..DIM {
            let xh = x - HALF;
            color = if xh*xh + yh*yh < HALF*HALF {
                lerp_nrgba(color, RED, DIST)
            } else {
                lerp_nrgba(color, WHITE, DIST)
            };
            buf[y*DIM + x] = color;
        }
    }
}

const fn lerp_nrgba(v0: Nrgba, v1: Nrgba, t: u8) -> Nrgba {
    let r = lerp(v0.r, v1.r, t);
    let g = lerp(v0.g, v1.g, t);
    let b = lerp(v0.b, v1.b, t);
    let a = lerp(v0.a, v1.a, t);
    Nrgba { r, g, b, a }
}

#[inline]
const fn lerp(v0: u8, v1: u8, t: u8) -> u8 {
    let (v0, v1, t) = (v0 as u32, v1 as u32, t as u32);
    let result = (v0*(255-t) + v1*t) / 255; result as u8
}
