use crate::{Space, Time};
use crate::space::{BaseUnit, ObservableSpace};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Seconds<B: BaseUnit>(B);

impl<B: BaseUnit> Time for Seconds<B> {}
impl<B: BaseUnit> Space for Seconds<B> {
    const DIMENSIONS: usize = 1;
    type Base = B;
}

impl<B: BaseUnit> ObservableSpace<1> for Seconds<B> {
    fn new(p: &[B; 1]) -> Self {
        Self(p[0])
    }

    fn components(&self) -> [Self; 1] {
        return [*self]
    }
}

impl<B: BaseUnit> core::ops::Add for Seconds<B> {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output { Self(self.0 + other.0) }
}

impl<B: BaseUnit> core::ops::Sub for Seconds<B> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output { Self(self.0 - other.0) }
}

impl<B: BaseUnit> core::ops::Div for Seconds<B> {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output { Self(self.0 / other.0) }
}

impl<B: BaseUnit> core::ops::Mul for Seconds<B> {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output { Self(self.0 * other.0) }
}