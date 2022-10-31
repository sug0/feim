use std::marker::PhantomData;

enum ImpossibleValue {}

pub struct No {
    _impossible_value: ImpossibleValue,
}

pub struct For<T> {
    _impossible_value: ImpossibleValue,
    _type: PhantomData<T>,
}

pub type Aye = For<()>;
