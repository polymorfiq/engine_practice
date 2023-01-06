use physics::Universe;
use physics::units::{space, time, mass};
use crate::bodies::enemies;

/// TODO: A basic, 2D Sidescroller universe
pub struct D2Basic {
    abs_time: time::Seconds,
    pub enemies: [enemies::BasicEnemy<Self>; 5]
}

impl Universe for D2Basic {
    type Space = space::Meters<2>;
    type Time = time::Seconds;
    type Mass = mass::Kilograms;

    fn time(&self) -> &time::Seconds {
        &self.abs_time
    }
}