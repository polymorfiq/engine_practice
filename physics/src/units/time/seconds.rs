use crate::{Space, Time};
use crate::space::{BaseUnit, Observable};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Seconds<B: BaseUnit>(B);

impl<B: BaseUnit> Seconds<B> {
    fn new(t: B) -> Self {
        Seconds(t)
    }
}

impl<B: BaseUnit> Time for Seconds<B> {}

impl<B: BaseUnit> Observable<1, Self> for Seconds<B> {
    fn new(p: &[B; 1]) -> Self {
        Self(p[0])
    }
}

impl<B: BaseUnit> Space<1> for Seconds<B> {
    type Base = B;

    fn area(&self) -> Self {
        *self
    }

    fn components(&self) -> [Self; 1] {
        return [*self]
    }

    fn distance(&self, b: &Self) -> Self {
        Self::new(b.0 - self.0)
    }

    fn offset(&self, offset: &Self) -> Self {
        Self::new(self.0 + offset.0)
    }

    fn scale(&self, b: &Self) -> Self {
        Self::new(self. 0 * b.0)
    }
}

// Required traits for Seconds -> Time
impl<B: BaseUnit> crate::Comparable for Seconds<B> {}
impl<B: BaseUnit> crate::Mobile for Seconds<B> {}