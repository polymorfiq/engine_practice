use core::default::Default;
use core::marker::Copy;
use core::ops;
use super::{Matrix, gen_matrix};

impl <T, const M: usize, const N: usize> ops::Add for Matrix<T, M, N>
    where T: ops::Add<Output = T> + Default + Copy
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        gen_matrix(|m, n| *self.value_at(m, n) + *other.value_at(m, n))
    }
}

impl <T, const M: usize, const N: usize> ops::Sub for Matrix<T, M, N>
    where T: ops::Sub<Output = T> + Default + Copy
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        gen_matrix(|m, n| *self.value_at(m, n) - *other.value_at(m, n))
    }
}

impl <T, const M: usize, const N: usize> ops::Mul<T> for Matrix<T, M, N>
    where T: Default + Copy + ops::Mul<Output = T>
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        gen_matrix(|m, n| *self.value_at(m, n) * other)
    }
}

impl <T, const M: usize, const N: usize> ops::Div<T> for Matrix<T, M, N>
    where T: Default + Copy + ops::Div<Output = T>
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        gen_matrix(|m, n| *self.value_at(m, n) / other)
    }
}