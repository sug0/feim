use std::io::{self, BufWriter};

use feim::buffer::{AsTypedMut, RawPixBuf};
use feim::color::Gray;
use feim::image::jpeg::{Jpeg, JpegEncodeOptions};
use feim::image::ImageMut;
use feim::serialize::EncodeSpecialized;

const DIM: usize = 2000;
const MAX_DEPTH: usize = 100;

struct Params<'a> {
    dir: GrowDir,
    buf: &'a mut RawPixBuf<Gray>,
    height: usize,
    depth: usize,
    cx: usize,
    cy: usize,
}

enum GrowDir {
    Left,
    Right,
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let image = {
        let mut image = RawPixBuf::new(DIM, DIM);
        draw_image(&mut image);
        image
    };

    let opts = JpegEncodeOptions::new(85).unwrap();
    Jpeg::encode_specialized(&mut stdout_writer, opts, &image)
}

const fn shade(y: u8) -> Gray {
    Gray { y }
}

const fn depth_shade(depth: usize) -> Gray {
    let y = (0xff * depth / MAX_DEPTH) << 3;
    shade((y & 0xff) as u8)
}

fn draw_image(buf: &mut RawPixBuf<Gray>) {
    // set image to white
    for pix in buf.as_typed_mut() {
        *pix = shade(0xff);
    }
    draw_image_recur(Params {
        dir: GrowDir::Right,
        height: DIM / 2,
        cx: (DIM - 1) / 2,
        cy: DIM - 1,
        depth: MAX_DEPTH,
        buf,
    });
}

// https://excalidraw.com/#json=YRtf0cDBfCknQZ9MkterY,p8lizfbHxbxLlpcvwB_Lrg
// https://www.todamateria.com.br/razoes-trigonometricas/
fn draw_image_recur(p: Params<'_>) {
    if p.depth == 0 || p.cx > DIM || p.cy > DIM {
        return;
    }
    match p.dir {
        GrowDir::Left => {
            let y_max = p.cy.saturating_sub(p.height);
            let x_max = p.cx.saturating_sub(p.height);
            let y_coords = (y_max..=p.cy).take_while(|&y| y < DIM);
            for y in y_coords {
                let x_coords = (x_max..=p.cx).take_while(|&x| x < DIM);
                for x in x_coords {
                    p.buf.pixel_set(x, y, depth_shade(p.depth));
                }
            }
            draw_image_recur(Params {
                dir: GrowDir::Right,
                height: p.height / 2,
                cx: x_max,
                cy: y_max,
                depth: p.depth - 1,
                buf: p.buf,
            });
        }
        GrowDir::Right => {
            let y_max = p.cy.saturating_sub(p.height);
            let x_max = p.cx.saturating_add(p.height);
            let y_coords = (y_max..=p.cy).take_while(|&y| y < DIM);
            for y in y_coords {
                let x_coords = (p.cx..=x_max).take_while(|&x| x < DIM);
                for x in x_coords {
                    p.buf.pixel_set(x, y, depth_shade(p.depth));
                }
            }
            draw_image_recur(Params {
                dir: GrowDir::Left,
                height: p.height / 2,
                cx: x_max,
                cy: y_max,
                depth: p.depth - 1,
                buf: p.buf,
            });
        }
    }
}
