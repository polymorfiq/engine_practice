use physics::{Quantifiable, Point, Positional, Intersects, Intersectable, Space};
use physics::units::space;

#[derive(Copy, Clone)]
pub struct Box<S: Space> {
    top_left: Point<S>,
    bottom_right: Point<S>
}

impl<const D: usize> Intersectable<space::Meters<D>> for Box<space::Meters<D>> {}

impl<const D: usize> Intersects<space::Meters<D>, Self> for Box<space::Meters<D>> {
    type Intersection = Self;

    fn distance_until_intersection(&self, _other: &Self) -> space::Meters<D> {
        space::Meters::new([0; D])
    }

    fn intersection(&self, _other: &Self) -> Self::Intersection {
        *self
    }
}

impl<const D: usize> Positional<space::Meters<D>> for Box<space::Meters<D>> {
    fn position(&self) -> &Point<space::Meters<D>> { &self.top_left }
}

impl<const D: usize> Quantifiable<space::Meters<D>> for Box<space::Meters<D>> {
    fn amount(&self) -> space::Meters<D> { self.top_left.distance(&self.bottom_right) }
}