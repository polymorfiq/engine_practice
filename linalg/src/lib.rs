#![no_std]

mod matrix;
pub use matrix::{Matrix, matrix};
pub type Scalar<T> = Matrix<T, 1, 1>;
pub type Vector<T, const L: usize> = Matrix<T, 1, L>;
