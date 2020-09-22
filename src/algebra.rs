pub trait SemiGroup {
    type T: Clone;
    fn op(lhs: &Self::T, rhs: &Self::T) -> Self::T;
}

pub trait Monoid {
    type T: Clone;
    fn identity() -> Self::T;
    fn op(lhs: &Self::T, rhs: &Self::T) -> Self::T;
}