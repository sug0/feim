use std::io::{self, BufReader, BufWriter};

use feim::buffer::RawPixBuf;
use feim::color::Nrgba64Be;
use feim::image::farbfeld::{Farbfeld, FarbfeldDecodeOptions};
use feim::image::{Dimensions, Image, ImageMut};
use feim::serialize::{Decode, EncodeSpecialized};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let stdin_reader = BufReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let stdout_writer = BufWriter::new(stdout_lock);

    let opts = FarbfeldDecodeOptions {
        check_header: false,
    };
    let image: RawPixBuf<Nrgba64Be> = Farbfeld::decode(stdin_reader, opts)?;
    Farbfeld::encode_specialized(stdout_writer, (), &blur(image))
}

fn blur(orig: RawPixBuf<Nrgba64Be>) -> RawPixBuf<Nrgba64Be> {
    let mut img = orig.clone();

    for y in 0..orig.height() {
        for x in 0..orig.width() {
            img.pixel_set(x, y, convolve(&orig, x, y));
        }
    }

    img
}

fn convolve(im: &RawPixBuf<Nrgba64Be>, x: usize, y: usize) -> Nrgba64Be {
    static KERN: [[f32; 3]; 3] = [
        [0.0625, 0.125, 0.0625],
        [0.1250, 0.250, 0.1250],
        [0.0625, 0.125, 0.0625],
    ];

    let mut accum = (0.0, 0.0, 0.0);

    for ky in -1..2 {
        for kx in -1..2 {
            let x = x as isize + kx;
            let y = y as isize + ky;
            let (r, g, b) = get_clamped(im, x, y);
            let mult = KERN[(ky + 1) as usize][(kx + 1) as usize];
            accum.0 = r.mul_add(mult, accum.0);
            accum.1 = g.mul_add(mult, accum.1);
            accum.2 = b.mul_add(mult, accum.2);
        }
    }

    Nrgba64Be::be(accum.0 as u16, accum.1 as u16, accum.2 as u16, 0xffff)
}

fn get_clamped(im: &RawPixBuf<Nrgba64Be>, x: isize, y: isize) -> (f32, f32, f32) {
    let w = im.width() as isize;
    let h = im.height() as isize;
    let x = x.clamp(0, w - 1) as usize;
    let y = y.clamp(0, h - 1) as usize;
    let c = im.color_get(x, y);
    (c.r() as f32, c.g() as f32, c.b() as f32)
}
