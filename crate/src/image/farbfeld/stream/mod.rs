use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, Write};
use std::sync::mpsc;

use super::Farbfeld;
use super::{Dimensions, Format};
use crate::color::{Nrgba64Be, Nrgba64Ne};
use crate::serialize::Encode;

pub struct FarbfeldPixelStream {
    width: u32,
    height: u32,
    pixels: mpsc::Receiver<Pixel>,
}

pub struct Pixel {
    x: u32,
    y: u32,
    color: Nrgba64Be,
}

impl PartialOrd for Pixel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.coords().cmp(&other.coords()))
    }
}

impl Ord for Pixel {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.x))
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        (self.x, self.y) == (other.x, other.y)
    }
}

impl Eq for Pixel {}

impl Pixel {
    pub fn bind(x: usize, y: usize, color: Nrgba64Be) -> Self {
        let x = x as u32;
        let y = y as u32;
        Self { x, y, color }
    }

    #[inline]
    fn coords(&self) -> u64 {
        ((self.x as u64) << 32) | (self.y as u64)
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

        while let Ok(pixel) = stream.pixels.recv() {
            let coords = pixel.coords();
            heap.push(pixel);
            if coords > window {
                continue;
            }
            while let Some(pixel) = heap.pop() {
                let c: Nrgba64Ne = pixel.color.cast();
                let c: u64 = c.into();
                let c = c.to_ne_bytes();
                w.write_all(&c[..])?;
                let coords = pixel.coords();
                if coords > window {
                    window = coords;
                    break;
                }
            }
        }
        while let Some(pixel) = heap.pop() {
            let c: Nrgba64Ne = pixel.color.cast();
            let c: u64 = c.into();
            let c = c.to_ne_bytes();
            w.write_all(&c[..])?;
        }

        Ok(())
    }
}
