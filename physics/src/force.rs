use crate::{Mass, Point, Space, Time};

pub trait Force<const D: usize, M: Mass, S: Space<D>, T: Time> {
    fn applied(&self, point: &dyn Point<D, S>, duration: &T) -> (M, S, T);
}