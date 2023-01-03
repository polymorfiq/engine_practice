mod arithmetic;
mod products;
mod constructors;

pub use constructors::{matrix, gen_matrix};

#[derive(Debug, Copy, Clone)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: [[T; N]; M],
}

impl <T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn rows(&self) -> usize { M }
    pub fn cols(&self) -> usize { N }

    pub fn value_at(&self, m: usize, n: usize) -> &T {
        &self.data[m][n]
    }

    pub fn raw(&self) -> &[[T; N]; M] {
        &self.data
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