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
    fn as_rgba(&self) -> (u32, u32, u32, u32);
}
