use crate::{Body, Force, Mass, Space, Time};

/// A (potentially mass-less) source of `Force` within a set `Space`
pub trait Field<const D: usize>: Space<D> {
    fn force<S: Space<D>, M: Mass, T: Time>(&self, b: dyn Body<D, S, M, T>) -> &dyn Force<D, M, Self, T>;
}