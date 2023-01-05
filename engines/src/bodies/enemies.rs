use physics::{Area, Body, Universe};
use physics::units::{space, time, mass};
use crate::universes::D2Basic;
use crate::areas;

pub struct RectangularEnemy {
    position: space::Meters2,
    bounding_box: areas::Box2
}

impl Body<D2Basic> for RectangularEnemy {
    fn boundary(&self) -> &dyn Area<space::Meters2> {
        &self.bounding_box
    }

    fn position(&self) -> space::Meters2 {
        self.position
    }

    fn distance(&self, other: &dyn Body<D2Basic>) -> space::Meters2 {
        other.position() - self.position
    }

    fn mass(&self) -> mass::Kilograms {
        mass::kilograms(5)
    }

    fn age(&self, universe: &D2Basic) -> time::Seconds {
        *universe.time()
    }
}