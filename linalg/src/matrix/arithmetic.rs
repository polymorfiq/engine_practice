use core::ops;
use crate::Mobile;
use super::{Matrix, gen_matrix};

impl <T: Mobile, const M: usize, const N: usize> ops::Add for Matrix<T, M, N>
    where T: ops::Add<Output = T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        gen_matrix(|m, n| *self.value_at(m, n) + *other.value_at(m, n))
    }
}

impl <T: Mobile, const M: usize, const N: usize> ops::Sub for Matrix<T, M, N>
    where T: ops::Sub<Output = T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        gen_matrix(|m, n| *self.value_at(m, n) - *other.value_at(m, n))
    }
}

impl <T: Mobile, const M: usize, const N: usize> ops::Div<T> for Matrix<T, M, N>
    where T: ops::Div<Output = T>
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        gen_matrix(|m, n| *self.value_at(m, n) / other)
    }
}

impl <T: Mobile, const M: usize, const N: usize> ops::Div<Self> for Matrix<T, M, N>
    where T: ops::Div<Output = T>
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        gen_matrix(|m, n| *self.value_at(m, n) / *other.value_at(m, n))
    }
}