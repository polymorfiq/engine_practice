use super::{Comparable, Mobile};

pub trait Space: Mobile + Comparable {
    const DIMENSIONS: usize;

    fn distance(a: &Point<Self>, b: &Point<Self>) -> Self;
    fn offset(point: &Point<Self>, offset: Self) -> Point<Self>;
}

pub trait Positional<S: Space> {
    fn position(&self) -> &Point<S>;
}

pub trait Quantifiable<S: Space> {
    fn amount(&self) -> S;
}

pub trait Intersectable<S: Space> {}
pub trait Intersects<S: Space, T> {
    type Intersection: Positional<S> + Quantifiable<S>;

    fn distance_until_intersection(&self, other: &T) -> S;
    fn intersection(&self, other: &T) -> Self::Intersection;
}

#[derive(Copy, Clone)]
pub struct Point<S: Space> {
    pub position: S
}

impl<S: Space> Point<S> {
    pub fn distance(&self, b: &Self) -> S { S::distance(self, b) }
    pub fn offset(&self, offset: S) -> Self { S::offset(self, offset) }
}

impl<S: Space> Positional<S> for Point<S> {
    fn position(&self) -> &Point<S> { &self }
}