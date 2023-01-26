use core::ops;
use crate::{Mobile, Vector};
use super::{Matrix, gen_matrix};

impl <T: Mobile, const M: usize, const N: usize> ops::Mul<T> for Matrix<T, M, N>
    where T: ops::Mul<Output = T>
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        gen_matrix(|m, n| *self.value_at(m, n) * other)
    }
}

impl <T: Mobile, const M: usize, const N: usize, const O: usize> ops::Mul<Matrix<T, N, O>> for Matrix<T, M, N>
    where T: ops::Mul<Output = T>
{
    type Output = Matrix<T, M, O>;

    fn mul(self, other: Matrix<T, N, O>) -> Self::Output {
        gen_matrix(|m, n| {
            let mut sum = T::zero();

            for row in 0..other.rows() {
                sum = sum + (*self.value_at(m, row) * *other.value_at(row, n));
            }
            sum
        })
    }
}

impl <T: Mobile, const L: usize> Vector<T, L>
    where T: ops::Mul<Output = T>
{
    pub fn vec_multiply(self, other: Vector<T, L>) -> Self {
        gen_matrix(|m, n| {
            *self.value_at(m, n) * *other.value_at(m, n)
        })
    }
}