use crate::Mass;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Kilograms(i64);
impl Kilograms {
    pub fn new(m: i64) -> Self { Kilograms(m) }
}

impl Mass for Kilograms {}