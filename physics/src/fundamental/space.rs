use super::{Divisible, Mobile};

pub trait Space<const D: usize>: Mobile + Divisible {}

pub trait Point<const D: usize, S: Space<D>> {
    fn distance(&self, other: dyn Point<D, S>) -> S;
    fn displaced(&self, space: S) -> dyn Point<D, S>;
}