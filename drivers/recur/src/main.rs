use std::io::{self, BufWriter};

use bracket_geometry::prelude::*;
use feim::buffer::{AsTypedMut, RawPixBuf};
use feim::color::Gray;
use feim::image::jpeg::{Jpeg, JpegEncodeOptions};
use feim::image::ImageMut;
use feim::serialize::EncodeSpecialized;

const DIM: usize = 500;
const MAX_DEPTH: usize = 8;
const ANGLE: f32 = 15.0;
const LENGTH: f32 = 50.0;
const LENGTH_FRAC: f32 = 0.8;

struct Params<'a> {
    buf: &'a mut RawPixBuf<Gray>,
    depth: usize,
    direction: f32,
    distance: f32,
    cx: f32,
    cy: f32,
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

fn draw_image(buf: &mut RawPixBuf<Gray>) {
    // set image to white
    for pix in buf.as_typed_mut() {
        *pix = shade(0xff);
    }
    draw_image_recur(Params {
        cx: (DIM / 2) as f32,
        cy: (DIM * 9 / 10) as f32,
        depth: MAX_DEPTH,
        distance: LENGTH,
        direction: 0.0,
        buf,
    });
}

fn draw_image_recur(params: Params<'_>) {
    let Params {
        buf,
        depth,
        direction,
        distance,
        cx: x1,
        cy: y1,
    } = params;

    if depth == 0 || x1 as usize > DIM || y1 as usize > DIM {
        return;
    }

    let (sin, cos) = (distance * std::f32::consts::PI / 180.0).sin_cos();
    let (x2, y2) = (x1 + distance * sin, y1 - distance * cos);

    let line_points = Bresenham::new(
        Point {
            x: x1 as i32,
            y: x2 as i32,
        },
        Point {
            x: x2 as i32,
            y: x2 as i32,
        },
    );

    for Point { x, y } in line_points {
        let x = x as usize;
        let y = y as usize;
        if x > DIM || y > DIM {
            return;
        }
        buf.pixel_set(x, y, shade(0));
    }

    let depth = depth - 1;
    let distance = distance * LENGTH_FRAC;

    draw_image_recur(Params {
        direction: direction - ANGLE,
        buf,
        depth,
        distance,
        cx: x2,
        cy: y2,
    });
    draw_image_recur(Params {
        direction: direction + ANGLE,
        buf,
        depth,
        distance,
        cx: x2,
        cy: y2,
    });
}
