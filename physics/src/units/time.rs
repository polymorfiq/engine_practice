use crate::{Point, Space, Time};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Seconds(i64);

impl Seconds {
    fn new(x: i64) -> Self {
        Seconds(x)
    }
}

impl Time for Seconds {}

impl Space for Seconds {
    const DIMENSIONS: usize = 1;

    fn distance(a: &Point<Self>, b: &Point<Self>) -> Self {
        Self::new(b.position.0 - a.position.0)
    }

    fn offset(point: &Point<Self>, offset: Self) -> Point<Self> {
        Point {position: Self::new(point.position.0 + offset.0)}
    }
}

// Required traits for Seconds -> Time
impl crate::Comparable for Seconds {}
impl crate::Mobile for Seconds {}