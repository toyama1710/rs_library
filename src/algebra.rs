pub trait Magma: Sized + Clone {
    fn op(&self, rhs: &Self) -> Self;
}

pub trait SemiGroup: Magma {}

pub trait Monoid: SemiGroup {
    fn identity() -> Self;
}