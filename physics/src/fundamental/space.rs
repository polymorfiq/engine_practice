use super::{Combineable, Comparable, Mobile};

pub trait BaseUnit: Combineable + Comparable + Mobile + linalg::Mobile {}
impl<T: Combineable + Comparable + Mobile + linalg::Mobile> BaseUnit for T {}

pub trait Space: Mobile + Comparable + Combineable {
    const DIMENSIONS: usize;
    type Base: BaseUnit;
}

pub trait ObservableSpace<const D: usize>: Space {
    fn components(&self) -> [Self; D];
    fn new(p: &[Self::Base; D]) -> Self;
}

pub trait Area<S: Space> {
    fn amount_of_space(&self) -> S;
}

pub trait AreaIntersection<S: Space, A: Area<S>>: Area<S> {
    fn area_intersection(&self, other: &A) -> Self;
}

pub trait SpaceIntersection<S: Space>: Area<S> {
    fn space_intersection(&self, other: &[S]) -> &[S];
}