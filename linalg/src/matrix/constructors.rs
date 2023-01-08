use crate::Mobile;
use super::Matrix;

pub fn gen_matrix<G, T, const M: usize, const N: usize>(gen: G) -> Matrix<T, M, N>
    where   G: Fn(usize, usize) -> T,
            T: Mobile
{
    let mut result: Matrix<T, M, N> = Matrix::zero();
    for m in 0..M {
        for n in 0..N {
            let val = gen(m, n);
            result.data[m][n] = val;
        }
    }
    
    result
}