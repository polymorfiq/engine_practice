#![no_std]
use core::marker::Copy;

mod matrix;
pub use matrix::Matrix;
pub type Scalar<T> = Matrix<T, 1, 1>;
pub type Vector<T, const L: usize> = Matrix<T, L, 1>;


impl <T: Mobile> Vector<T, 3> {
    pub fn vec4(v: Vector<T, 3>, item: T) -> Vector<T, 4> {
        Matrix { data: [[v.data[0][0], v.data[1][0], v.data[2][0], item]] }.transpose()
    }
}

impl <T: Mobile> Vector<T, 4> {
    pub fn vec3(&self) -> Vector<T, 3> {
        Matrix { data: [[self.data[0][0], self.data[1][0], self.data[2][0]]] }.transpose()
    }
}

impl <T: Mobile, const L: usize> Vector<T, L> {
    pub fn vector(data: [T; L]) -> Self {
        Matrix { data: [data] }.transpose()
    }
}

pub trait Mobile: Zero + Copy + core::ops::Add<Output = Self> {}
impl<T: Zero + Copy + core::ops::Add<Output = T>> Mobile for T {}

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for f32 { fn zero() -> Self { 0.0 } }
impl Zero for f64 { fn zero() -> Self { 0.0 } }