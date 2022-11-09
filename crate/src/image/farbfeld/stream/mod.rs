use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::io::{self, Write};

use super::Farbfeld;
use super::{Dimensions, Format};
use crate::color::{Color, Nrgba64Be, Nrgba64Ne};
use crate::image::{ConvertInto, ImageMut};
use crate::serialize::Encode;

#[derive(Debug, Copy, Clone)]
pub struct Params {
    pub buffer_cap: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub struct FarbfeldPixelStream {
    width: u32,
    height: u32,
    pixels: kanal::Receiver<Pixel>,
}

enum RollOver {
    Next { window: u64 },
    Eof,
}

#[derive(Debug, Clone)]
pub struct FarbfeldPixelStreamPusher {
    tx: kanal::Sender<Pixel>,
}

impl ImageMut for FarbfeldPixelStreamPusher {
    type Pixel = Nrgba64Be;

    fn color_set<P, ColorSpecialized>(&mut self, x: usize, y: usize, color: P)
    where
        P: ConvertInto<Nrgba64Be, ColorSpecialized> + Color,
    {
        let color: Nrgba64Be = color.convert_into();
        self.tx
            .send(Pixel::bind(x, y, color))
            .expect("FarbfeldPixelStream was closed!")
    }
}

impl FarbfeldPixelStream {
    pub fn new(params: Params) -> (FarbfeldPixelStreamPusher, Self) {
        let (tx, rx) = kanal::bounded(params.buffer_cap);
        let rx = Self {
            width: params.width as u32,
            height: params.height as u32,
            pixels: rx,
        };
        let tx = FarbfeldPixelStreamPusher { tx };
        (tx, rx)
    }

    fn roll_over(&self, window: u64) -> RollOver {
        let x = (window & 0xffffffff) as u32;
        let y = (window >> 32) as u32;

        let new_x = (x + 1) % self.width;
        let new_y = if new_x != 0 { y } else { y + 1 };

        if new_y < self.height {
            RollOver::Next {
                window: get_coords(new_x, new_y),
            }
        } else {
            RollOver::Eof
        }
    }
}

struct Pixel {
    x: u32,
    y: u32,
    color: Nrgba64Be,
}

impl PartialOrd for Pixel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pixel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.coords().cmp(&other.coords())
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        (self.x, self.y) == (other.x, other.y)
    }
}

impl Eq for Pixel {}

impl Pixel {
    fn bind(x: usize, y: usize, color: Nrgba64Be) -> Self {
        let x = x as u32;
        let y = y as u32;
        Self { x, y, color }
    }

    #[inline]
    fn coords(&self) -> u64 {
        get_coords(self.x, self.y)
    }
}

impl Dimensions for FarbfeldPixelStream {
    fn width(&self) -> usize {
        self.width as usize
    }

    fn height(&self) -> usize {
        self.height as usize
    }
}

impl Encode<FarbfeldPixelStream> for Farbfeld {
    fn encode<W: Write>(mut w: W, _opts: (), stream: &FarbfeldPixelStream) -> io::Result<()> {
        // write header
        {
            let width = stream.width.to_be_bytes();
            let height = stream.height.to_be_bytes();
            let magic = Farbfeld.magic();
            w.write_all(&magic[..8])?;
            w.write_all(&width[..])?;
            w.write_all(&height[..])?;
        }

        let mut window = 0;
        let mut heap = BinaryHeap::new();

        'recvpix: loop {
            let Ok(pixel) = stream.pixels.recv() else {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "Encoding ended early"));
            };
            if pixel.x >= stream.width || pixel.y >= stream.height {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Pixel out of bounds: ({}, {})", pixel.x, pixel.y),
                ));
            }
            let coords = pixel.coords();
            if heap
                .peek()
                .map(|Reverse(pix): &Reverse<Pixel>| pix.coords() > coords)
                .unwrap_or(false)
            {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Attempted to encode the same pixel twice: ({}, {})",
                        pixel.x, pixel.y
                    ),
                ));
            }
            heap.push(Reverse(pixel));
            if coords > window {
                continue;
            }
            'bufwrite: while let Some(Reverse(pixel)) = heap.pop() {
                let c: Nrgba64Ne = pixel.color.cast();
                let c: u64 = c.into();
                let c = c.to_ne_bytes();
                w.write_all(&c[..])?;
                match stream.roll_over(pixel.coords()) {
                    RollOver::Next { window: coords } if coords > window => {
                        window = coords;
                        break 'bufwrite;
                    }
                    RollOver::Next { .. } => (),
                    RollOver::Eof => break 'recvpix,
                }
            }
        }

        Ok(())
    }
}

#[inline]
fn get_coords(x: u32, y: u32) -> u64 {
    ((y as u64) << 32) | (x as u64)
}
