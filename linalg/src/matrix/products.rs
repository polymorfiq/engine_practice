use core::default::Default;
use core::marker::Copy;
use core::ops;
use super::{Matrix, gen_matrix};

impl <T, const M: usize, const N: usize> Matrix<T, M, N>
    where T: Default + Copy + core::iter::Sum + ops::Mul<Output = T>
{
    pub fn dot<const O: usize>(&self, b: &Matrix<T, N, O>) -> Matrix<T, M, O> {
        gen_matrix(|m, o| {
            (0..N).map(|n| *self.value_at(m, n) * *b.value_at(n, o)).sum()
        })
    }
}