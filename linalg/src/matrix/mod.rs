pub use crate::{Mobile, Zero};

mod arithmetic;
pub use arithmetic::*;

mod products;
pub use products::*;

mod constructors;
use constructors::gen_matrix;

#[derive(Debug, Copy, Clone)]
pub struct Matrix<T: Mobile, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

impl <T: Mobile, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(data: [[T; N]; M]) -> Self {
        Self { data: data }
    }


    pub fn row_major(&self) -> [[T; N]; M] { self.data }
    pub fn col_major(&self) -> [[T; M]; N] { self.transpose().data }

    pub fn rows(&self) -> usize { M }
    pub fn cols(&self) -> usize { N }

    pub fn set(&mut self, m: usize, n: usize, val: T) {
        self.data[m][n] = val;
    }

    pub fn value_at(&self, m: usize, n: usize) -> &T {
        &self.data[m][n]
    }

    pub fn row(&self, m: usize) -> Matrix<T, 1, N> {
        gen_matrix(|_, n| self.data[m][n])
    }

    pub fn col(&self, n: usize) -> Matrix<T, M, 1> {
        gen_matrix(|m, _| self.data[m][n])
    }

    pub fn transpose(&self) -> Matrix<T, N, M> {
        gen_matrix(|m, n| self.data[n][m])
    }

    fn zero() -> Self {
        Matrix { data: [[T::zero(); N]; M] }
    }
}