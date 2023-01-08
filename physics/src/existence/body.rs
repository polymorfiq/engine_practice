use crate::Universe;
use crate::space::Quantifiable;

/// A particular `Mass` moving through a `Space` and `Time`
pub trait Body<const D: usize, U: Universe<D>> {
    type Boundary: Quantifiable<D, U::Space>;

    fn boundary(&self) -> Self::Boundary;
    fn position(&self) -> U::Space;
    fn mass(&self) -> U::Mass;
}