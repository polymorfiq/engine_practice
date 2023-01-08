use crate::{Force, Universe};
use crate::space::{Intersects, Intersectable};

pub trait Field<const D: usize, U: Universe<D>, F: Force<D, U>> {
    type Bounds: Intersectable<D, U::Space>;
    type Force: Force<D, U>;

    fn force<I: Intersects<D, U::Space, Self::Bounds>>(&self, other: &I) -> F;
}