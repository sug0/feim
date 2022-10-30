use std::io::{self, BufReader, BufWriter};

use feim::buffer::RawPixBuf;
use feim::color::{Nrgba64Be, Nrgba64Ne};
use feim::image::{
    farbfeld::Farbfeld,
    jpeg::{Jpeg, JpegBuf},
    png::Png,
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
            let image: RawPixBuf<Nrgba64Ne> = image.encode_as();
            let opts = Default::default();
            Png::encode(stdout_writer, opts, &image)
        }
        Ok(1) => {
            let image = Jpeg::decode(stdin_reader, ())?;
            let opts = Default::default();

            match &image {
                JpegBuf::Gray(buf) => Png::encode(stdout_writer, opts, buf),
                JpegBuf::Gray16(buf) => Png::encode(stdout_writer, opts, buf),
                JpegBuf::Rgb(buf) => Png::encode(stdout_writer, opts, buf),
                JpegBuf::Cmyk(_) => todo!(),
            }
        }
        Ok(2) => {
            let image = Png::decode(stdin_reader, ())?;
            let opts = Default::default();
            Png::encode(stdout_writer, opts, &image)
        }
        Ok(_) => unreachable!(),
        Err(e) => Err(e),
    }
}
