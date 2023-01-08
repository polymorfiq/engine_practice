use super::{Space, Time, Mass};

/// A `Universe` represents the combination of a particular `Mass` and `Space` and `Time`.
pub trait Universe<const SPATIAL_DIMENSIONS: usize> {
    type Space: Space<SPATIAL_DIMENSIONS>;
    type Time: Time;
    type Mass: Mass;

    fn time(&self) -> &Self::Time;
}