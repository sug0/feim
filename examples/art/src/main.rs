use std::io::{self, BufWriter};

use feim::buffer::{AsTypedMut, RawPixBuf};
use feim::color::Gray;
use feim::image::jpeg::{Jpeg, JpegEncodeOptions};
use feim::serialize::Encode;

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

    let opts = JpegEncodeOptions::new(85).unwrap();
    Jpeg::encode(&mut stdout_writer, opts, &image)
}

const fn shade(y: u8) -> Gray {
    Gray { y }
}

fn draw_image(buf: &mut [Gray]) {
    for y in 0..DIM {
        let yf = y as f32;
        for x in 0..DIM {
            let xf = x as f32;
            let value = 255.0 * (xf - std::f32::consts::PI * xf).sin();
            let v = ((value as usize) << (x ^ !y))
                ^ (value.cos().mul_add(yf, std::f32::consts::PI * xf.cos()) as usize);
            buf[y * DIM + x] = shade((v & 0xff) as u8);
        }
    }
}