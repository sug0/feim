use std::convert::Infallible;
use std::marker::PhantomData;

pub struct No(Infallible);

pub struct Yes<T> {
    _impossible_value: Infallible,
    _type: PhantomData<T>,
}

pub type Aye = Yes<()>;
