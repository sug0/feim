use std::io::{self, BufReader, BufWriter};

use feim::buffer::RawPixBuf;
use feim::color::convert::ConvertInto;
use feim::color::{Gray, Nrgba64Be};
use feim::image::farbfeld::{Farbfeld, FarbfeldDecodeOptions};
use feim::image::{Dimensions, Image, ImageMut};
use feim::serialize::{Decode, EncodeSpecialized};
use feim::specialized;

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
    Farbfeld::encode_specialized(stdout_writer, (), &bentley(image))
}

fn bentley(orig: RawPixBuf<Nrgba64Be>) -> RawPixBuf<Nrgba64Be> {
    let mut img = orig.clone();

    for y in 0..orig.height() {
        for x in 0..orig.width() {
            let brightness: Gray =
                <_ as ConvertInto<Gray, specialized::No>>::convert_into(orig.color_get(x, y));
            let scaled_brightness = (orig.height() / 255) * brightness.y as usize;
            let y_bentley = y.saturating_sub(scaled_brightness / 8);
            img.pixel_set(x, y, orig.color_get(x, y_bentley));
        }
    }

    img
}
