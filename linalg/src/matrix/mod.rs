mod arithmetic;
mod products;
mod constructors;

use constructors::gen_matrix;

#[derive(Debug, Copy, Clone)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

impl <T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn new(data: [[T; N]; M]) -> Self {
        Self { data: data }
    }

    pub fn rows(&self) -> usize { M }
    pub fn cols(&self) -> usize { N }

    pub fn value_at(&self, m: usize, n: usize) -> &T {
        &self.data[m][n]
    }
}

impl <T: Default + Copy, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn row(&self, m: usize) -> Matrix<T, 1, N> {
        gen_matrix(|_, n| self.data[m][n])
    }

    pub fn col(&self, n: usize) -> Matrix<T, M, 1> {
        gen_matrix(|m, _| self.data[m][n])
    }
}

impl<T: Default + Copy, const M: usize, const N: usize> Default for Matrix<T, M, N> {
    fn default() -> Self {
        Matrix { data: [[Default::default(); N]; M] }
    }
}