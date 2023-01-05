use linalg::{Matrix, matrix};
use crate::Space;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Meters(i64);
pub fn meters(x: i64) -> Meters { Meters(x) }
impl Space for Meters {
    const D: usize = 1;
}

// Required traits for Meters -> Space
impl crate::Divisible for Meters {}
impl crate::Mobile for Meters {}
impl core::default::Default for Meters {
    fn default() -> Self { Self(0) }
}
impl core::ops::Add for Meters {
    type Output = Self;
    fn add(self, other: Self) -> Self { Self(self.0 + other.0) }
}
impl core::ops::Sub for Meters {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self(self.0 - other.0) }
}
impl core::ops::Div for Meters {
    type Output = Self;
    fn div(self, other: Self) -> Self { Self(self.0 / other.0) }
}
impl core::ops::Mul for Meters {
    type Output = Self;
    fn mul(self, other: Self) -> Self { Self(self.0 * other.0) }
}

#[derive(Copy, Clone)]
pub struct Meters2(Matrix<Meters, 1, 2>);
pub fn meters2(x: i64, y: i64) -> Meters2 { Meters2(matrix([[Meters(x), Meters(y)]])) }
impl Space for Meters2 {
    const D: usize = 2;
}

// Required traits for Meters2 -> Space
impl crate::Divisible for Meters2 {}
impl crate::Mobile for Meters2 {}
impl core::ops::Add for Meters2 {
    type Output = Self;
    fn add(self, other: Self) -> Self { Self(self.0 + other.0) }
}
impl core::ops::Sub for Meters2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self(self.0 - other.0) }
}
impl core::ops::Div for Meters2 {
    type Output = Self;
    fn div(self, other: Self) -> Self { Self(self.0 / other.0) }
}
impl core::ops::Mul for Meters2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self { Self(self.0 * other.0) }
}
impl core::cmp::PartialOrd for Meters2 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl core::cmp::Ord for Meters2 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let my_mag = *self.0.value_at(0, 0) * *self.0.value_at(0, 1);
        let other_mag = *other.0.value_at(0, 0) * *other.0.value_at(0, 1);
        my_mag.cmp(&other_mag)
    }
}
impl core::cmp::Eq for Meters2 {}
impl core::cmp::PartialEq for Meters2 {
    fn eq(&self, other: &Self) -> bool {
        (*self.0.value_at(0, 0) == *other.0.value_at(0, 0)) &&
        (*self.0.value_at(0, 1) == *other.0.value_at(0, 1))
    }
}

#[derive(Copy, Clone)]
pub struct Meters3(Matrix<Meters, 1, 3>);
pub fn meters3(x: i64, y: i64, z: i64) -> Meters3 { Meters3(matrix([[Meters(x), Meters(y), Meters(z)]])) }
impl Space for Meters3 {
    const D: usize = 3;
}

// Required traits for Meters3 -> Space
impl crate::Divisible for Meters3 {}
impl crate::Mobile for Meters3 {}
impl core::ops::Add for Meters3 {
    type Output = Self;
    fn add(self, other: Self) -> Self { Self(self.0 + other.0) }
}
impl core::ops::Sub for Meters3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self(self.0 - other.0) }
}
impl core::ops::Div for Meters3 {
    type Output = Self;
    fn div(self, other: Self) -> Self { Self(self.0 / other.0) }
}
impl core::ops::Mul for Meters3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self { Self(self.0 * other.0) }
}
impl core::cmp::PartialOrd for Meters3 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl core::cmp::Ord for Meters3 {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let my_mag = *self.0.value_at(0, 0) * *self.0.value_at(0, 1) * *self.0.value_at(0, 2);
        let other_mag = *other.0.value_at(0, 0) * *other.0.value_at(0, 1) * *other.0.value_at(0, 2);
        my_mag.cmp(&other_mag)
    }
}
impl core::cmp::Eq for Meters3 {}
impl core::cmp::PartialEq for Meters3 {
    fn eq(&self, other: &Self) -> bool {
        (*self.0.value_at(0, 0) == *other.0.value_at(0, 0)) &&
        (*self.0.value_at(0, 1) == *other.0.value_at(0, 1)) &&
        (*self.0.value_at(0, 2) == *other.0.value_at(0, 2))
    }
}