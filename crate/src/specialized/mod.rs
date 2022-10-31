use std::marker::PhantomData;

enum ImpossibleValue {}

pub struct NotSpecialized {
    _impossible_value: ImpossibleValue,
}

pub struct SpecializedFor<T> {
    _impossible_value: ImpossibleValue,
    _type: PhantomData<T>,
}

pub struct Specialized {
    _inner: SpecializedFor<()>,
}

pub type For<T> = SpecializedFor<T>;

pub type No = NotSpecialized;

pub type Aye = Specialized;
