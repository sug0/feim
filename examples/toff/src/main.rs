use std::io::{self, BufReader, BufWriter};

use feim::buffer::RawPixBuf;
use feim::color::Nrgba64Be;
use feim::image::{
    self,
    farbfeld::{Farbfeld, FarbfeldDecodeOptions},
    jpeg::{Jpeg, JpegBuf},
    png::{Png, PngBuf},
    webp::Webp,
    BuiltInFormat,
};
use feim::serialize::{try_format, Decode, Encode, EncodeSpecialized};

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
            Farbfeld::encode_specialized(stdout_writer, (), &image)
        }
        Ok(BuiltInFormat::Jpeg) => {
            let image = Jpeg::decode(stdin_reader, ())?;

            match &image {
                JpegBuf::Gray(buf) => Farbfeld::encode(stdout_writer, (), buf),
                JpegBuf::Gray16(buf) => Farbfeld::encode(stdout_writer, (), buf),
                JpegBuf::Rgb(buf) => Farbfeld::encode(stdout_writer, (), buf),
                JpegBuf::Cmyk(buf) => Farbfeld::encode(stdout_writer, (), buf),
            }
        }
        Ok(BuiltInFormat::Png) => {
            let image = Png::decode(stdin_reader, ())?;

            match &image {
                PngBuf::Gray(buf) => Farbfeld::encode(stdout_writer, (), buf),
                PngBuf::Gray16(buf) => Farbfeld::encode(stdout_writer, (), buf),
                PngBuf::Nrgba(buf) => Farbfeld::encode(stdout_writer, (), buf),
                PngBuf::Nrgba64(buf) => Farbfeld::encode_specialized(stdout_writer, (), buf),
                PngBuf::Rgb(buf) => Farbfeld::encode(stdout_writer, (), buf),
                PngBuf::Rgb48(buf) => Farbfeld::encode(stdout_writer, (), buf),
            }
        }
        Ok(BuiltInFormat::Webp) => {
            let image = Webp::decode(stdin_reader, ())?;

            match &image {
                either::Left(rgb_buf) => Farbfeld::encode(stdout_writer, (), rgb_buf),
                either::Right(nrgba_buf) => Farbfeld::encode(stdout_writer, (), nrgba_buf),
            }
        }
        Err(e) => Err(e),
    }
}
