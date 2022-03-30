pub mod convert;

mod rgb;
mod cmyk;
mod nrgba;
mod nrgba64;
mod gray;
mod gray16;

pub use rgb::*;
pub use cmyk::*;
pub use nrgba::*;
pub use nrgba64::*;
pub use gray::*;
pub use gray16::*;

pub trait Color {
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
