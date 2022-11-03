use std::io::{self, BufReader, BufWriter};

use feim::buffer::RawPixBuf;
use feim::color::Nrgba64Be;
use feim::image::{
    self,
    farbfeld::{Farbfeld, FarbfeldDecodeOptions},
    jpeg::{Jpeg, JpegBuf},
    png::Png,
    webp::Webp,
    BuiltInFormat,
};
use feim::serialize::{try_format, Decode, EncodeSpecialized};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut stdin_reader = BufReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let stdout_writer = BufWriter::new(stdout_lock);

    match try_format(&mut stdin_reader, image::built_in_formats_iter()) {
        Ok(BuiltInFormat::Farbfeld) => {
            let opts = FarbfeldDecodeOptions {
                check_header: false,
            };
            let image: RawPixBuf<Nrgba64Be> = Farbfeld::decode(stdin_reader, opts)?;
            let opts = Default::default();
            Png::encode_specialized(stdout_writer, opts, &image)
        }
        Ok(BuiltInFormat::Jpeg) => {
            let image = Jpeg::decode(stdin_reader, ())?;
            let opts = Default::default();

            match &image {
                JpegBuf::Gray(buf) => Png::encode_specialized(stdout_writer, opts, buf),
                JpegBuf::Gray16(buf) => Png::encode_specialized(stdout_writer, opts, buf),
                JpegBuf::Rgb(buf) => Png::encode_specialized(stdout_writer, opts, buf),
                JpegBuf::Cmyk(_) => todo!(),
            }
        }
        Ok(BuiltInFormat::Png) => {
            let image = Png::decode(stdin_reader, ())?;
            let opts = Default::default();
            Png::encode_specialized(stdout_writer, opts, &image)
        }
        Ok(BuiltInFormat::Webp) => {
            let image = Webp::decode(stdin_reader, ())?;
            let opts = Default::default();

            match &image {
                either::Left(rgb_buf) => Png::encode_specialized(stdout_writer, opts, rgb_buf),
                either::Right(nrgba_buf) => Png::encode_specialized(stdout_writer, opts, nrgba_buf),
            }
        }
        Err(e) => Err(e),
    }
}
