use crate::Universe;
use crate::units::{space, time, mass};

/// TODO: A basic, 2D Sidescroller universe
pub struct D2Basic {}

impl Universe<2, space::Meters2, time::Seconds, mass::Kilograms> for D2Basic {
    fn mass(&self) -> mass::Kilograms { mass::kilograms(0) }
    fn time(&self) -> time::Seconds { time::seconds(0) }
    fn space(&self) -> space::Meters2 { space::meters2(0, 0) }
}