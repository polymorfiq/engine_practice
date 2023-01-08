use crate::Universe;

/// The movement of `Mass` through `Space` over an amount of `Time`
pub trait Force<const D: usize, U: Universe<D>> {
    fn applied(&self, point: &U::Space, duration: &U::Time) -> (U::Mass, U::Space, U::Time);
}