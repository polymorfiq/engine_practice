use physics::Universe;
use physics::units::{space, time, mass};
use crate::bodies::enemies;

/// TODO: A basic, 2D Sidescroller universe
pub struct D2Basic {
    abs_time: time::Seconds,
    enemies: [enemies::RectangularEnemy; 5]
}

impl Universe for D2Basic {
    type S = space::Meters2;
    type T = time::Seconds;
    type M = mass::Kilograms;

    fn time(&self) -> &time::Seconds {
        &self.abs_time
    }
}