use std::marker::PhantomData;

pub enum No {}

pub struct Yes<T> {
    _marker: PhantomData<T>,
}

pub type Aye = Yes<()>;
