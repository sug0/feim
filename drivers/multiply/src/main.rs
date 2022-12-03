use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

use feim::buffer::RawPixBuf;
use feim::color::Nrgba64Be;
use feim::image::farbfeld::{Farbfeld, FarbfeldDecodeOptions};
use feim::image::{Dimensions, Image, ImageMut};
use feim::serialize::{Decode, EncodeSpecialized};

enum Chan {
    R,
    G,
    B,
    A,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        usage()?;
    }

    let fst_file = File::open(&args[1])?;
    let fst_file_reader = BufReader::new(fst_file);

    let snd_file = File::open(&args[2])?;
    let snd_file_reader = BufReader::new(snd_file);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let stdout_writer = BufWriter::new(stdout_lock);

    let opts = FarbfeldDecodeOptions {
        check_header: false,
    };
    let fst_image: RawPixBuf<Nrgba64Be> = Farbfeld::decode(fst_file_reader, opts)?;
    let snd_image: RawPixBuf<Nrgba64Be> = Farbfeld::decode(snd_file_reader, opts)?;

    Farbfeld::encode_specialized(stdout_writer, (), &multiply(fst_image, snd_image)?)?;
    Ok(())
}

fn usage() -> Result<(), &'static str> {
    Err("Need to provide two image args")
}

fn multiply(
    fst: RawPixBuf<Nrgba64Be>,
    snd: RawPixBuf<Nrgba64Be>,
) -> Result<RawPixBuf<Nrgba64Be>, &'static str> {
    if snd.width() != fst.height() {
        return Err("Width of second image is not the same as first image's height");
    }

    let mut output = RawPixBuf::new(snd.width(), fst.height());

    for i in 0..output.height() {
        for j in 0..output.width() {
            let r = sum_channel(&fst, &snd, Chan::R, i, j);
            let g = sum_channel(&fst, &snd, Chan::G, i, j);
            let b = sum_channel(&fst, &snd, Chan::B, i, j);
            let a = sum_channel(&fst, &snd, Chan::A, i, j);
            output.pixel_set(j, i, Nrgba64Be::be(r, g, b, a));
        }
    }

    Ok(output)
}

fn sum_channel(
    fst: &RawPixBuf<Nrgba64Be>,
    snd: &RawPixBuf<Nrgba64Be>,
    which: Chan,
    i: usize,
    j: usize,
) -> u16 {
    let sum: u64 = (0..fst.width())
        .map(|k| {
            let c1 = fst.color_get(k, i);
            let c2 = snd.color_get(j, k);

            let (c1, c2) = match which {
                Chan::R => ((c1.r() >> 8) as u64, (c2.r() >> 8) as u64),
                Chan::G => ((c1.g() >> 8) as u64, (c2.g() >> 8) as u64),
                Chan::B => ((c1.b() >> 8) as u64, (c2.b() >> 8) as u64),
                Chan::A => ((c1.a() >> 8) as u64, (c2.a() >> 8) as u64),
            };

            ((c1 * c2) << 8) & 0xffff
        })
        .sum();
    (sum & 0xffff) as u16
}
