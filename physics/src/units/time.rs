use crate::Time;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Seconds(i64);
pub fn seconds(s: i64) -> Seconds { Seconds(s) }
impl Time for Seconds {}

// Required traits for Seconds -> Time
impl crate::Divisible for Seconds {}
impl crate::Mobile for Seconds {}
impl core::ops::Add for Seconds {
    type Output = Self;
    fn add(self, other: Self) -> Self { Self(self.0 + other.0) }
}
impl core::ops::Sub for Seconds {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self(self.0 - other.0) }
}
impl core::ops::Div for Seconds {
    type Output = Self;
    fn div(self, other: Self) -> Self { Self(self.0 / other.0) }
}
impl core::ops::Mul for Seconds {
    type Output = Self;
    fn mul(self, other: Self) -> Self { Self(self.0 * other.0) }
}