#![no_std]
use core::marker::Copy;

mod matrix;
pub use matrix::Matrix;
pub type Scalar<T> = Matrix<T, 1, 1>;
pub type Vector<T, const L: usize> = Matrix<T, 1, L>;

pub trait Mobile: Zero + Copy {}
impl<T: Zero + Copy> Mobile for T {}

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for f32 { fn zero() -> Self { 0.0 } }
impl Zero for f64 { fn zero() -> Self { 0.0 } }