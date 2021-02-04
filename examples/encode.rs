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
    colorize(image.as_typed_mut());

    Farbfeld.encode(&mut stdout_writer, &&mut image)
}

fn colorize(buf: &mut [Nrgba]) {
    for y in 0..DIM {
        for x in 0..DIM {
            buf[y*DIM + x] = Nrgba { r: 255, g: 0, b: 0, a: 255 };
        }
    }
}
