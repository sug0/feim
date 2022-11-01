mod expression;

use std::io::{self, BufWriter};

use feim::buffer::RawPixBuf;
use feim::color::Nrgba64Be;
use feim::image::farbfeld::Farbfeld;
use feim::image::ImageMut;
use feim::serialize::EncodeSpecialized;

const DIM: usize = 1000;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let mut image = RawPixBuf::new(DIM, DIM);
    draw_image(&mut image);

    Farbfeld::encode_specialized(&mut stdout_writer, (), &image)
}

fn draw_image(buf: &mut RawPixBuf<Nrgba64Be>) {
    const MAX: u16 = 0xffff;
    const BLACK: Nrgba64Be = Nrgba64Be::be(0, 0, 0, MAX);
    const WHITE: Nrgba64Be = Nrgba64Be::be(MAX, MAX, MAX, MAX);

    for y in 0..DIM {
        for x in 0..DIM {
            let cond = x > y;
            buf.pixel_set(x, y, if cond { BLACK } else { WHITE });
        }
    }
}
