use core::marker::Copy;
use core::ops::{Add, Sub, Div, Mul};
use core::cmp::{Eq, Ord};

pub trait Mobile: Copy {}
pub trait Divisible: Eq + Ord + Add + Sub + Div + Mul + Sized {}