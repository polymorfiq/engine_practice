use core::marker::Copy;
use core::cmp::{Eq, Ord};
use core::ops::{Add, Sub, Div, Mul};

pub trait Comparable: Eq + Ord {}
impl<T: Eq + Ord> Comparable for T {}

pub trait Mobile: Copy + Sized {}
impl<T: Copy + Sized> Mobile for T {}

pub trait Combineable: Mobile + Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + Mul<Output = Self> {}
impl<T: Mobile + Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + Mul<Output = Self>> Combineable for T {}