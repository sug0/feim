use std::io::{self, BufReader, BufWriter};

use feim::buffer::RawPixBuf;
use feim::color::convert::ConvertInto;
use feim::color::{Gray, Nrgba64Be};
use feim::image::farbfeld::{Farbfeld, FarbfeldDecodeOptions};
use feim::image::{Dimensions, Image, ImageMut};
use feim::serialize::{DecodeSpecialized, EncodeSpecialized};
use feim::specialized;

struct Mask<'a> {
    width: usize,
    height: usize,
    m: u16,
    pix: &'a [u16],
}

// bayer mask
const MASK: Mask<'static> = Mask {
    width: 4,
    height: 4,
    m: 16,
    pix: &[1, 9, 3, 11, 13, 5, 15, 7, 4, 12, 2, 10, 16, 8, 14, 6],
};

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
    let image: RawPixBuf<Nrgba64Be> = Farbfeld::decode_specialized(stdin_reader, opts)?;
    Farbfeld::encode_specialized(stdout_writer, (), &halftone(image))
}

fn halftone(orig: RawPixBuf<Nrgba64Be>) -> RawPixBuf<Nrgba64Be> {
    let mut img = RawPixBuf::new(orig.width(), orig.height());

    for y in (0..orig.height()).step_by(MASK.height) {
        for x in (0..orig.width()).step_by(MASK.width) {
            MASK.apply(&orig, &mut img, x, y);
        }
    }

    img
}

impl Mask<'_> {
    fn apply(
        &self,
        orig: &RawPixBuf<Nrgba64Be>,
        im: &mut RawPixBuf<Nrgba64Be>,
        x: usize,
        y: usize,
    ) {
        const BLACK: Nrgba64Be = Nrgba64Be::be(0, 0, 0, 0xffff);
        const WHITE: Nrgba64Be = Nrgba64Be::be(0xffff, 0xffff, 0xffff, 0xffff);

        let w_max = im.width() - 1;
        let h_max = im.height() - 1;

        for i in 0..self.height {
            let cy = (y + i).clamp(0, h_max);
            for j in 0..self.width {
                let cx = (x + j).clamp(0, w_max);
                let color: Gray =
                    <_ as ConvertInto<Gray, specialized::No>>::convert_into(orig.color_get(cx, cy));
                let pix = color.y as u16 + self.pix[i * self.width + j] * self.m;
                im.pixel_set(cx, cy, if pix > 0xff { WHITE } else { BLACK });
            }
        }
    }
}
