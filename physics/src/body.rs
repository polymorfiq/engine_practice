use crate::{Area, Universe};

/// A particular `Mass` moving through a `Space` and `Time`
pub trait Body<U: Universe> {
    fn boundary(&self) -> &dyn Area<U::S>;
    fn position(&self) -> U::S;
    fn distance(&self, other: &dyn Body<U>) -> U::S;
    fn mass(&self) -> U::M;
    fn age(&self, universe: &U) -> U::T;
}