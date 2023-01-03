use core::ops;
use linalg::Vector;

pub type Distance<T> = Vector<T, 3>;
type Coordinates<T> = Vector<T, 3>;

pub struct Position<T> {
    coords: Coordinates<T>,
}

impl<T> Position<T>
    where T: Default + Copy + ops::Sub<Output = T>
{
    pub fn direction(&self, other: &Position<T>) -> Distance<T> {
        other.coords - self.coords
    }
}