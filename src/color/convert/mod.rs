use super::Color;

pub trait ConvertFrom<C: Color> {
    /// Returns the result of converting a color from `C` into
    /// `Self`.
    fn convert_from(c: C) -> Self;
}

pub trait ConvertInto<C> {
    /// Returns the result of converting a color from `Self` into
    /// `C`.
    fn convert_into(self) -> C;
}

impl<A, B> ConvertInto<B> for A
where
    A: Color,
    B: ConvertFrom<A>,
{
    default fn convert_into(self) -> B {
        B::convert_from(self)
    }
}
