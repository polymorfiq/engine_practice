use super::{Space, Time, Mass};

pub trait Universe<const D: usize, S: Space<D>, T: Time, M: Mass> {
    fn space(&self) -> S;
    fn time(&self) -> T;
    fn mass(&self) -> M;
}