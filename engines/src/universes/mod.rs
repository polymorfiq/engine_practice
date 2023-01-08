use physics::Universe;
use physics::units::{space, time, mass};
use crate::bodies::enemies;

pub struct D2Basic {
    abs_time: time::Seconds<i64>,
    pub enemies: [enemies::BasicEnemy<2, Self>; 5]
}

impl Universe<2> for D2Basic {
    type Space = space::Meters<2, i64>;
    type Time = time::Seconds<i64>;
    type Mass = mass::Kilograms;

    fn time(&self) -> &Self::Time {
        &self.abs_time
    }
}