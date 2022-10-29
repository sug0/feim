pub mod convert;

mod cmyk;
mod gray;
mod gray16;
mod nrgba;
mod nrgba64;
mod rgb;
mod rgb48;

use crate::specialized::No;

pub use cmyk::*;
pub use gray::*;
pub use gray16::*;
pub use nrgba::*;
pub use nrgba64::*;
pub use rgb::*;
pub use rgb48::*;

pub trait Color<Specialized = No> {
    /// Return an alpha premultiplied color in RGBA 16 bits.
    ///
    /// Components are returned as native-endian values.
    fn as_rgba(&self) -> (u32, u32, u32, u32);
}

// -------------------------------------------------------------------------- //

/// Tag a pixel's channel value as big-endian.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BigEndian {}

/// Tag a pixel's channel value as little-endian.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LittleEndian {}

/// Tag a pixel's channel value as native-endian.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NativeEndian {}

// -------------------------------------------------------------------------- //

pub(crate) trait Endianness {}

impl Endianness for BigEndian {}

impl Endianness for LittleEndian {}

impl Endianness for NativeEndian {}
