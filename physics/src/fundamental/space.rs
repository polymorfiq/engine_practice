use super::{Combineable, Comparable, Mobile};

pub trait BaseUnit: Combineable + Comparable + Mobile + linalg::Mobile {}
impl<T: Combineable + Comparable + Mobile + linalg::Mobile> BaseUnit for T {}

pub trait Space: Mobile + Comparable {
    const DIMENSIONS: usize;
    type Base: BaseUnit;

    fn area(&self) -> Self;
    fn distance(&self, b: &Self) -> Self;
    fn offset(&self, offset: &Self) -> Self;
    fn scale(&self, b: &Self) -> Self;
}

pub trait Observable<const D: usize>: Space {
    fn components(&self) -> [Self; D];
    fn new(p: &[Self::Base; D]) -> Self;
}

pub trait Positional<S: Space> {
    fn position(&self) -> &Self;
}

pub trait Quantifiable<S: Space> {
    fn area(&self) -> S;
}

pub trait Intersectable<S: Space> {}
pub trait Intersects<S: Space, T: Intersectable<S>> {
    type Intersection: Quantifiable<S>;

    fn distance_until_intersection(&self, other: &T) -> S;
    fn intersection(&self, other: &T) -> Self::Intersection;
}