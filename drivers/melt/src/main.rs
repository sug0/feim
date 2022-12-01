mod prng;

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
    Farbfeld::encode_specialized(stdout_writer, (), &melt(image))
}

fn melt(mut im: RawPixBuf<Nrgba64Be>) -> RawPixBuf<Nrgba64Be> {
    let random_pairs = NextTwo {
        iterator: prng::State::new().map(|x| x as usize),
    };

    let width = im.width();
    let height = im.height();
    let height_1 = height - 1;

    for (r0, r1) in random_pairs.take(width * height) {
        let x = r0 % width;
        let y0 = r1 % height_1;

        for y in y0..height_1 {
            let fst = im.color_get(x, y);
            let snd = im.color_get(x, y + 1);

            if u64::from(fst) > u64::from(snd) {
                break;
            }

            im.pixel_set(x, y, snd);
            im.pixel_set(x, y + 1, fst);
        }
    }

    im
}

struct NextTwo<I> {
    iterator: I,
}

impl<I: Iterator> Iterator for NextTwo<I> {
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iterator.next()?;
        let y = self.iterator.next()?;
        Some((x, y))
    }
}
