use crate::Universe;

/// The movement of `Mass` through `Space` over an amount of `Time`
pub trait Force<U: Universe> {
    fn applied(&self, point: &U::Space, duration: &U::Time) -> (U::Mass, U::Space, U::Time);
}