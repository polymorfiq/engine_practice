use super::{Space, Time, Mass};

/// A `Universe` represents the combination of a particular `Mass` and `Space` and `Time`.
pub trait Universe {
    type S: Space;
    type T: Time;
    type M: Mass;

    fn time(&self) -> &Self::T;
}