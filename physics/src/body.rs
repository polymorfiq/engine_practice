use crate::{Mass, Space, Time};

pub trait Body<'a, const D: usize, S: Space<D>, M: Mass, T: Time> {
    fn boundary(&self) -> S;
    fn distance(&self, other: &'a dyn Body<'a, D, S, M, T>) -> S;
    fn mass(&self) -> M;
    fn age(self) -> T;
}