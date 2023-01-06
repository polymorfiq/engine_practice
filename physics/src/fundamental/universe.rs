use super::{Space, Time, Mass};

/// A `Universe` represents the combination of a particular `Mass` and `Space` and `Time`.
pub trait Universe {
    type Space: Space;
    type Time: Time;
    type Mass: Mass;

    fn time(&self) -> &Self::Time;
}