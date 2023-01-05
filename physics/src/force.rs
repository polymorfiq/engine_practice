use crate::Universe;

/// The movement of `Mass` through `Space` over an amount of `Time`
pub trait Force<U: Universe> {
    fn applied(&self, point: &U::S, duration: &U::T) -> (U::M, U::S, U::T);
}