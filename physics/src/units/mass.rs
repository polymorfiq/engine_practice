use crate::Mass;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kilograms(i64);
pub fn kilograms(m: i64) -> Kilograms { Kilograms(m) }
impl Mass for Kilograms {}

// Required traits for Kilograms -> Mass
impl crate::Divisible for Kilograms {}
impl crate::Mobile for Kilograms {}
impl core::ops::Add for Kilograms {
    type Output = Self;
    fn add(self, other: Self) -> Self { Self(self.0 + other.0) }
}
impl core::ops::Sub for Kilograms {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self(self.0 - other.0) }
}
impl core::ops::Div for Kilograms {
    type Output = Self;
    fn div(self, other: Self) -> Self { Self(self.0 / other.0) }
}
impl core::ops::Mul for Kilograms {
    type Output = Self;
    fn mul(self, other: Self) -> Self { Self(self.0 * other.0) }
}