use super::{Divisible, Mobile};

/// `Space` is the fundamental dimension of Position, Distance, Direction, and Area
/// 
/// `Space` contains `Mass`, and is itself moving through `Time`.
pub trait Space: Mobile + Divisible {
    const D: usize;
}

/// An `Area` is a subset of a `Space`
pub trait Area<S: Space> {
    fn intersects(&self, space: &S) -> bool;
}