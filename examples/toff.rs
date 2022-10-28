use std::io::{self, BufReader, BufWriter};

use feim::buffer::RawPixBuf;
use feim::color::{Cmyk, Gray, Gray16Ne, Nrgba, Nrgba64Be, Nrgba64Ne, Rgb, Rgb48Ne};
use feim::image::{
    farbfeld::Farbfeld,
    jpeg::{Jpeg, JpegBuf},
    png::{Png, PngBuf},
    Format,
};
use feim::serialize::{try_format, Decode, Encode, GenericDecodeOptions};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut stdin_reader = BufReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let stdout_writer = BufWriter::new(stdout_lock);

    let formats: [&dyn Format; 3] = [
        &Farbfeld, &Jpeg, &Png,
        // ...
    ];

    match try_format(&mut stdin_reader, &formats[..]) {
        Ok(0) => {
            let opts = GenericDecodeOptions {
                check_header: false,
            };
            let image: RawPixBuf<Nrgba64Be> = Farbfeld::decode(stdin_reader, opts)?;
            Farbfeld::encode(stdout_writer, (), &image)
        }
        Ok(1) => {
            let image = Jpeg::decode(stdin_reader, ())?;

            match &image {
                JpegBuf::Gray(buf) => {
                    <Farbfeld as Encode<RawPixBuf<Gray>>>::encode(stdout_writer, (), buf)
                }
                JpegBuf::Gray16(buf) => {
                    <Farbfeld as Encode<RawPixBuf<Gray16Ne>>>::encode(stdout_writer, (), buf)
                }
                JpegBuf::Rgb(buf) => {
                    <Farbfeld as Encode<RawPixBuf<Rgb>>>::encode(stdout_writer, (), buf)
                }
                JpegBuf::Cmyk(buf) => {
                    <Farbfeld as Encode<RawPixBuf<Cmyk>>>::encode(stdout_writer, (), buf)
                }
            }
        }
        Ok(2) => {
            let image = Png::decode(stdin_reader, ())?;

            match &image {
                PngBuf::Gray(buf) => {
                    <Farbfeld as Encode<RawPixBuf<Gray>>>::encode(stdout_writer, (), buf)
                }
                PngBuf::Gray16(buf) => {
                    <Farbfeld as Encode<RawPixBuf<Gray16Ne>>>::encode(stdout_writer, (), buf)
                }
                PngBuf::Nrgba(buf) => {
                    <Farbfeld as Encode<RawPixBuf<Nrgba>>>::encode(stdout_writer, (), buf)
                }
                PngBuf::Nrgba64(buf) => {
                    <Farbfeld as Encode<RawPixBuf<Nrgba64Ne>>>::encode(stdout_writer, (), buf)
                }
                PngBuf::Rgb(buf) => {
                    <Farbfeld as Encode<RawPixBuf<Rgb>>>::encode(stdout_writer, (), buf)
                }
                PngBuf::Rgb48(buf) => {
                    <Farbfeld as Encode<RawPixBuf<Rgb48Ne>>>::encode(stdout_writer, (), buf)
                }
            }
        }
        Ok(_) => unreachable!(),
        Err(e) => Err(e),
    }
}
