use std::io::{self, BufWriter};

use feim::buffer::{AsTypedMut, RawPixBuf};
use feim::color::{BigEndian, Nrgba64};
use feim::image::farbfeld::Farbfeld;
use feim::serialize::EncodeSpecialized;

const DIM: usize = 1000;

type Nrgba64Be = Nrgba64<BigEndian>;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let mut image = RawPixBuf::new(DIM, DIM);
    draw_image(image.as_typed_mut());

    Farbfeld::encode_specialized(&mut stdout_writer, (), &image)
}

fn draw_image(buf: &mut [Nrgba64Be]) {
    const MAX: u16 = 0xffff;
    const HALF: usize = DIM / 2;
    const BLACK: Nrgba64Be = Nrgba64::be(0, 0, 0, MAX);
    const WHITE: Nrgba64Be = Nrgba64::be(MAX, MAX, MAX, MAX);

    for y in 0..DIM {
        let yt = y.next_power_of_two();
        let yh = y - HALF;
        for x in 0..DIM {
            let xt = x.next_power_of_two();
            let xh = x - HALF;
            let cond = ((xt * yt) + (xh * (yt - yh))) < ((xh >> yt) % (!(yh << xh) | 1));
            buf[y * DIM + x] = if cond { BLACK } else { WHITE };
        }
    }
}
