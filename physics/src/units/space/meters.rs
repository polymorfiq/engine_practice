use linalg::Matrix;
use crate::Space;
use crate::space::{BaseUnit, ObservableSpace};

#[derive(Copy, Clone)]
pub struct Meters<const DIMENSIONS: usize, B: BaseUnit>(Matrix<B, 1, DIMENSIONS>);

impl<const D: usize, B: BaseUnit> Space for Meters<D, B> {
    const DIMENSIONS: usize = D;
    type Base = B;
}

impl<const D: usize, B: BaseUnit> ObservableSpace<D> for Meters<D, B> {
    fn new(p: &[B; D]) -> Self {
        Self(Matrix::new([*p]))
    }

    fn components(&self) -> [Self; D] {
        let mut comps: [Self; D] = [Self::new(&[B::zero(); D]); D];
        for i in 0..D {
            comps[i].0.set(0, i, *self.0.value_at(0, i))
        }

        comps
    }
}

impl<const D: usize, B: BaseUnit> core::cmp::PartialOrd for Meters<D, B> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<const D: usize, B: BaseUnit> core::cmp::Ord for Meters<D, B> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let mut magnitude_a = B::zero();
        let mut magnitude_b = B::zero();
        for i in 0..D {
            magnitude_a = magnitude_a + (*self.0.value_at(0, i) * *self.0.value_at(0, i));
            magnitude_b = magnitude_b + (*other.0.value_at(0, i) * *other.0.value_at(0, i));
        }

        magnitude_a.cmp(&magnitude_b)
    }
}
impl<const D: usize, B: BaseUnit> core::cmp::Eq for Meters<D, B> {}
impl<const D: usize, B: BaseUnit> core::cmp::PartialEq for Meters<D, B> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..D {
            if *self.0.value_at(0, i) != *other.0.value_at(0, i) {
                return false
            }
        }

        true
    }
}

impl<const D: usize, B: BaseUnit> core::ops::Add for Meters<D, B> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output { Self(self.0 + other.0) }
}

impl<const D: usize, B: BaseUnit> core::ops::Sub for Meters<D, B> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output { Self(self.0 - other.0) }
}

impl<const D: usize, B: BaseUnit> core::ops::Div for Meters<D, B> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output { Self(self.0 / other.0) }
}

impl<const D: usize, B: BaseUnit> core::ops::Mul for Meters<D, B> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output { Self(self.0 * other.0) }
}