use super::{Comparable, Mobile};
use core::ops::{Add, Sub, Div, Mul};
use core::cmp::{Eq, Ord};

pub trait BaseUnit: Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + Mul<Output = Self> + Eq + Ord + Copy + Clone + Sized + linalg::Mobile {}
impl<T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T> + Eq + Ord + Copy + Clone + Sized + linalg::Mobile> BaseUnit for T {}

pub trait Space<const D: usize>: Mobile + Comparable {
    type Base: BaseUnit;

    fn components(&self) -> [Self; D];
    fn area(&self) -> Self;
    fn distance(&self, b: &Self) -> Self;
    fn offset(&self, offset: &Self) -> Self;
    fn scale(&self, b: &Self) -> Self;
}

pub trait Observable<const D: usize, S: Space<D>> {
    fn new(p: &[S::Base; D]) -> S;
}

pub trait Positional<const D: usize, S: Space<D>> {
    fn position(&self) -> &Self;
}

pub trait Quantifiable<const D: usize, S: Space<D>> {
    fn area(&self) -> S;
}

pub trait Intersectable<const D: usize, S: Space<D>> {}
pub trait Intersects<const D: usize, S: Space<D>, T: Intersectable<D, S>> {
    type Intersection: Quantifiable<D, S>;

    fn distance_until_intersection(&self, other: &T) -> S;
    fn intersection(&self, other: &T) -> Self::Intersection;
}