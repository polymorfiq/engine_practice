use core::ops;
use linalg::Vector;

pub trait Distance<T> {}

type Coordinates<T> = Vector<T, 3>;

pub struct Position<T> {
    coords: Coordinates<T>,
}

impl<T> Position<T>
    where T: Default + Copy + ops::Sub<Output = T>
{
    pub fn direction(&self, other: &Position<T>) -> Coordinates<T> {
        other.coords - self.coords
    }
}