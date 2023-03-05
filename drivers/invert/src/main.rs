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
    Farbfeld::encode_specialized(stdout_writer, (), &invert(image))
}

fn invert(mut im: RawPixBuf<Nrgba64Be>) -> RawPixBuf<Nrgba64Be> {
    for y in 0..im.height() {
        for x in 0..im.width() {
            let c = im.color_get(x, y);
            im.pixel_set(
                x,
                y,
                Nrgba64Be::be(c.r() ^ 0xffff, c.g() ^ 0xffff, c.b() ^ 0xffff, c.a()),
            );
        }
    }
    im
}
