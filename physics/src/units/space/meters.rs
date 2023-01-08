use linalg::Matrix;
use crate::Space;
use crate::space::{BaseUnit, Observable};

#[derive(Copy, Clone)]
pub struct Meters<const DIMENSIONS: usize, B: BaseUnit>(Matrix<B, 1, DIMENSIONS>);

impl<const D: usize, B: BaseUnit> Observable<D, Self> for Meters<D, B> {
    fn new(p: &[B; D]) -> Self {
        Self(Matrix::new([*p]))
    }
}

impl<const D: usize, B: BaseUnit> Space<D> for Meters<D, B> {
    type Base = B;

    fn components(&self) -> [Self; D] {
        let mut comps: [Self; D] = [Self::new(&[B::zero(); D]); D];
        for i in 0..D {
            comps[i].0.set(0, i, *self.0.value_at(0, i))
        }

        comps
    }

    fn area(&self) -> Self {
        *self
    }

    fn distance(&self, b: &Self) -> Self {
        let mut new_vals: [B; D] = [B::zero(); D];

        for i in 0..D {
            new_vals[i] = *b.0.value_at(0, i) - *self.0.value_at(0, i);
        }

        Self::new(&new_vals)
    }

    fn offset(&self, offset: &Self) -> Self {
        let mut new_vals: [B; D] = [B::zero(); D];
        for i in 0..D {
            new_vals[i] = *self.0.value_at(0, i) + *offset.0.value_at(0, i);
        }

        Self::new(&new_vals)
    }

    fn scale(&self, b: &Self) -> Self {
        let mut other_s = B::zero();
        for i in 0..D {
            other_s = other_s + *b.0.value_at(0, i)
        }

        let mut new_vals: [B; D] = [B::zero(); D];
        for i in 0..D {
            new_vals[i] = *self.0.value_at(0, i) * other_s;
        }

        Self::new(&new_vals)
    }
}

impl<const D: usize, B: BaseUnit> crate::Comparable for Meters<D, B> {}
impl<const D: usize, B: BaseUnit> crate::Mobile for Meters<D, B> {}

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
    
    fn add(self, rhs: Self) -> Self::Output {
        let mut area = Self::new(&[B::zero(); D]);
        area.0 = self.0 + rhs.0;
        area
    }
}