use std::io::{self, BufReader, BufWriter, Write};

use clap::Parser;
use feim::buffer::RawPixBuf;
use feim::color::convert::ConvertInto;
use feim::color::{Gray, Nrgba64Be};
use feim::image::farbfeld::{Farbfeld, FarbfeldDecodeOptions};
use feim::image::{Dimensions, Image};
use feim::serialize::Decode;
use feim::specialized;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CmdLineArgs {
    /// The luminance value
    #[arg(short, long, default_value_t = 128)]
    luminance: u8,
    /// Whether the luminance value shall be inverted
    #[arg(short, long, default_value_t = false)]
    inverted: bool,
}

fn main() -> io::Result<()> {
    let args = CmdLineArgs::parse();

    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let stdin_reader = BufReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let stdout_writer = BufWriter::new(stdout_lock);

    let opts = FarbfeldDecodeOptions {
        check_header: false,
    };
    let image = Farbfeld::decode(stdin_reader, opts)?;
    args.braille(image, stdout_writer)
}

impl CmdLineArgs {
    fn braille<W: Write>(&self, im: RawPixBuf<Nrgba64Be>, mut w: W) -> io::Result<()> {
        for y in (0..im.height()).step_by(4) {
            for x in (0..im.width()).step_by(2) {
                let mut bits = 0u32;
                let mut output = [0u8; 4];

                bits |= self.lum_at(&im, x, y);
                bits |= self.lum_at(&im, x + 1, y) << 3;
                bits |= self.lum_at(&im, x, y + 1) << 1;
                bits |= self.lum_at(&im, x + 1, y + 1) << 4;
                bits |= self.lum_at(&im, x, y + 2) << 2;
                bits |= self.lum_at(&im, x + 1, y + 2) << 5;
                bits |= self.lum_at(&im, x, y + 3) << 6;
                bits |= self.lum_at(&im, x + 1, y + 3) << 7;
                bits |= 0x2800;

                let bits = unsafe {
                    // SAFETY: we have a valid braille char
                    // at this point
                    char::from_u32_unchecked(bits)
                };
                let braille_char = bits.encode_utf8(&mut output[..]);

                w.write_all(braille_char.as_ref())?;
            }
            w.write_all(b"\n")?;
        }
        Ok(())
    }

    #[inline]
    fn lum_at(&self, im: &RawPixBuf<Nrgba64Be>, x: usize, y: usize) -> u32 {
        #[inline(always)]
        fn at(im: &RawPixBuf<Nrgba64Be>, x: usize, y: usize) -> Gray {
            <_ as ConvertInto<Gray, specialized::No>>::convert_into(im.color_get(x, y))
        }
        let thres = at(im, x, y).y < self.luminance;
        (thres != self.inverted) as u32
    }
}
