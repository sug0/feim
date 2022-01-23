use std::io::{self, BufWriter};

use feim::buffer::{RawPixBuf, AsTypedMut};
use feim::image::farbfeld::Farbfeld;
use feim::serialize::Encode;
use feim::color::Nrgba64;

const DIM: usize = 1000;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let mut image = RawPixBuf::new(DIM, DIM);
    draw_image(image.as_typed_mut());

    Farbfeld::encode(&mut stdout_writer, (), &image)
}

fn draw_image(buf: &mut [Nrgba64]) {
    const MAX: u16 = 0xffff;
    const HALF: usize = DIM / 2;
    const BLACK: Nrgba64 = Nrgba64 { r: 0, g: 0, b: 0, a: MAX };
    const WHITE: Nrgba64 = Nrgba64 { r: MAX, g: MAX, b: MAX, a: MAX };

    for y in 0..DIM {
        let yt = y.next_power_of_two();
        let yh = y - HALF;
        for x in 0..DIM {
            let xt = x.next_power_of_two();
            let xh = x - HALF;
            let cond = ((xt*yt)+(xh*(yt-yh))) < ((xh>>yt)%(!(yh<<xh)|1));
            buf[y*DIM + x] = if cond { BLACK } else { WHITE };
        }
    }
}
