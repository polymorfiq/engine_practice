use crate::{Quantifiable, Universe};

/// A particular `Mass` moving through a `Space` and `Time`
pub trait Body<U: Universe> {
    type Boundary: Quantifiable<U::Space>;

    fn boundary(&self) -> Self::Boundary;
    fn position(&self) -> U::Space;
    fn mass(&self) -> U::Mass;
}