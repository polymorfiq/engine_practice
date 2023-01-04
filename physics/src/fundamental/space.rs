use super::{Divisible, Mobile};

pub trait Space<const D: usize>: Mobile + Divisible {
    fn overlap(&self, other: Self) -> Self;
    fn contains(&self, p: &dyn Point<D, Self>) -> bool;
}

pub trait Point<const D: usize, S: Space<D>> {
    fn distance(&self, other: dyn Point<D, S>) -> S;
    fn displaced(&self, space: S) -> dyn Point<D, S>;
}