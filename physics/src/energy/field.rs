use crate::{Force, Universe};
use crate::space::{Intersects, Intersectable};

pub trait Field<U: Universe, F: Force<U>> {
    type Bounds: Intersectable<U::Space>;
    type Force: Force<U>;

    fn force<I: Intersects<U::Space, Self::Bounds>>(&self, other: &I) -> F;
}