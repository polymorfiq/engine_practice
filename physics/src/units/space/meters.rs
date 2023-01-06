use linalg::Matrix;
use crate::{Point, Space};

#[derive(Copy, Clone)]
pub struct Meters<const DIMENSIONS: usize>(Matrix<i64, 1, DIMENSIONS>);

impl<const D: usize> Meters<D> {
    pub fn new(m: [i64; D]) -> Self { Self(Matrix::new([m])) }
}

impl<const D: usize> Space for Meters<D> {
    const DIMENSIONS: usize = D;

    fn distance(a: &Point<Self>, b: &Point<Self>) -> Self {
        let mut new_vals = [0; D];
        for i in 0..D {
            new_vals[i] = b.position.0.value_at(0, i) - a.position.0.value_at(0, i);
        }

        Self::new(new_vals)
    }

    fn offset(point: &Point<Self>, offset: Self) -> Point<Self> {
        let mut new_vals = [0; D];
        for i in 0..D {
            new_vals[i] = point.position.0.value_at(0, i) + offset.0.value_at(0, i);
        }

        Point{position: Self::new(new_vals)}
    }
}

impl<const D: usize> crate::Comparable for Meters<D> {}
impl<const D: usize> crate::Mobile for Meters<D> {}
impl<const D: usize> core::default::Default for Meters<D> {
    fn default() -> Self { Self(Matrix::new([[0; D]])) }
}

impl<const D: usize> core::cmp::PartialOrd for Meters<D> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<const D: usize> core::cmp::Ord for Meters<D> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let mut magnitude_a = 0;
        let mut magnitude_b = 0;
        for i in 0..D {
            magnitude_a = magnitude_a + (*self.0.value_at(0, i) * *self.0.value_at(0, i));
            magnitude_b = magnitude_b + (*other.0.value_at(0, i) * *other.0.value_at(0, i));
        }

        magnitude_a.cmp(&magnitude_b)
    }
}
impl<const D: usize> core::cmp::Eq for Meters<D> {}
impl<const D: usize> core::cmp::PartialEq for Meters<D> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..D {
            if *self.0.value_at(0, i) != *other.0.value_at(0, i) {
                return false
            }
        }

        true
    }
}