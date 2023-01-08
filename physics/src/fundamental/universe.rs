use super::{Space, Time, Mass};

pub trait Universe {
    type Space: Space;
    type Time: Time;
    type Mass: Mass;

    fn time(&self) -> &Self::Time;
}