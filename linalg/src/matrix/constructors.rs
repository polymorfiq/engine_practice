use core::default::Default;
use core::marker::Copy;
use super::Matrix;

pub fn matrix<T, const M: usize, const N: usize>(data: [[T; N]; M]) -> Matrix<T, M, N> {
    Matrix { data: data }
}

pub fn gen_matrix<G, T, const M: usize, const N: usize>(gen: G) -> Matrix<T, M, N>
    where   G: Fn(usize, usize) -> T,
            T: Default + Copy
{
    let mut result: Matrix<T, M, N> = Default::default();
    for m in 0..M {
        for n in 0..N {
            let val = gen(m, n);
            result.data[m][n] = val;
        }
    }
    
    result
}