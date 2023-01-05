use super::{Divisible, Mobile};

/// `Space` is the fundamental dimension of Position, Distance, Direction, and Area
/// `Space` contains `Mass`, and is itself moving through `Time`.
pub trait Space<const D: usize>: Mobile + Divisible {}

/// A `Point` is an infinitesimal position within `Space`
pub trait Point<const D: usize, S: Space<D>> {
    fn to(&self, other: &dyn Point<D, S>) -> &dyn Displacement<D, S, dyn Point<D, S>>;
}

/// A `Displacement` represents an path from one `Point` to another `Point`
pub trait Displacement<const D: usize, S: Space<D>, P: Point<D, S>> {
    fn from(&self, point: P) -> P;
}

/// An `Area` is a subset of a `Space`
pub trait Area<const D: usize, S: Space<D>> {
    fn contains(&self, point: dyn Point<D, S>) -> bool;
    fn intersects(&self, displacement: dyn Displacement<D, S, dyn Point<D, S>>) -> bool;
}