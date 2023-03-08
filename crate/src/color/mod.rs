pub mod convert;

mod cmyk;
mod gray;
mod gray16;
mod nrgba;
mod nrgba64;
mod rgb;
mod rgb48;

pub use cmyk::*;
pub use gray::*;
pub use gray16::*;
pub use nrgba::*;
pub use nrgba64::*;
pub use rgb::*;
pub use rgb48::*;

/// A color which contains a zero value.
pub trait Zero: Color + Copy {
    /// The zero value of this color.
    const ZERO: Self;
}

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

#[doc(hidden)]
pub trait EndiannessPrivate {}

pub trait Endianness: EndiannessPrivate {}

impl EndiannessPrivate for BigEndian {}

impl EndiannessPrivate for LittleEndian {}

impl EndiannessPrivate for NativeEndian {}

impl<E: EndiannessPrivate> Endianness for E {}
