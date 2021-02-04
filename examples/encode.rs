use std::io::{self, BufWriter};

use feim::image::{
    Encode,
    farbfeld::Farbfeld,
};
use feim::buffer::RawPixBuf;
use feim::color::Nrgba;

const DIM: usize = 500;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let mut image = RawPixBuf::new(DIM, DIM);
    draw_image(image.as_typed_mut());

    Farbfeld.encode(&mut stdout_writer, &&mut image)
}

fn draw_image(buf: &mut [Nrgba]) {
    const HALF: usize = DIM / 2;
    const RED: Nrgba = Nrgba { r: 255, g: 0, b: 0, a: 255 };
    const WHITE: Nrgba = Nrgba { r: 255, g: 255, b: 255, a: 255 };

    for y in 0..DIM {
        let yh = y - HALF;
        for x in 0..DIM {
            let xh = x - HALF;
            if xh*xh + yh*yh < HALF*HALF {
                buf[y*DIM + x] = RED;
            } else {
                buf[y*DIM + x] = WHITE;
            }
        }
    }
}
