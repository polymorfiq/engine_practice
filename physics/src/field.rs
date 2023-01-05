use crate::{Area, Body, Force, Universe};

/// A (potentially mass-less) source of `Force` within a set `Space`
pub trait Field<U: Universe>: Area<U::S> {
    type F: Force<U>;

    fn force(&self, body: dyn Body<U>) -> &dyn Force<U>;
}