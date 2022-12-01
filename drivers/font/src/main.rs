use std::env;
use std::io::{self, BufWriter};

use feim::buffer::{AsTypedMut, RawPixBuf};
use feim::color::Gray;
use feim::image::png::{Png, PngEncodeOptions};
use feim::image::ImageMut;
use feim::serialize::EncodeSpecialized;
use fontdue::Font;

const FONT_SIZE: f32 = 250.0;
const DIM: usize = 600;
const TEXT: &str = "420";

fn main() -> io::Result<()> {
    let font = {
        let args: Vec<_> = env::args().collect();

        if args.len() < 2 {
            eprintln!("Usage: {} <font>", args[0]);
            return Ok(());
        }

        let font_data = std::fs::read(&args[1])?;

        Font::from_bytes(font_data, Default::default())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
    };

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let image = {
        let mut image = RawPixBuf::new(DIM, DIM);
        draw_image(font, &mut image);
        image
    };

    let opts = PngEncodeOptions::default();
    Png::encode_specialized(&mut stdout_writer, opts, &image)
}

const fn shade(y: u8) -> Gray {
    Gray { y }
}

fn draw_image(font: Font, buf: &mut RawPixBuf<Gray>) {
    // set image to white
    for pix in buf.as_typed_mut() {
        *pix = shade(0xff);
    }

    let (mut x0, y0) = (DIM / 6, DIM / 3);

    for ch in TEXT.chars() {
        let (metrics, bitmap) = font.rasterize(ch, FONT_SIZE);

        for i in 0..metrics.height {
            for j in 0..metrics.width {
                buf.pixel_set(x0 + j, y0 + i, shade(0xff ^ bitmap[i * metrics.width + j]));
            }
        }

        x0 += metrics.width;
    }
}
