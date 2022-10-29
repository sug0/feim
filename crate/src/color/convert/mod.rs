use super::Color;
use crate::specialized::No;

pub trait ConvertFrom<C, Specialized = No>
where
    C: Color,
{
    /// Returns the result of converting a color from `C` into
    /// `Self`.
    fn convert_from(c: C) -> Self;
}

pub trait ConvertInto<C, Specialized = No>
where
    C: Color,
{
    /// Returns the result of converting a color from `Self` into
    /// `C`.
    fn convert_into(self) -> C;
}

impl<A, B, Specialized> ConvertInto<B, Specialized> for A
where
    A: Color,
    B: ConvertFrom<A, Specialized> + Color,
{
    fn convert_into(self) -> B {
        B::convert_from(self)
    }
}
