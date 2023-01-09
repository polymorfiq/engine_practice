use crate::Universe;
use crate::space::Area;

/// A particular `Mass` moving through a `Space` and `Time`
pub trait Body<U: Universe> {
    type Boundary: Area<U::Space>;

    fn boundary(&self) -> Self::Boundary;
    fn position(&self) -> U::Space;
    fn mass(&self) -> U::Mass;
}