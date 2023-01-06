use physics::{Body, Universe};
use physics::units::{space, mass};
use crate::areas;

pub struct BasicEnemy<U: Universe> {
    position: U::Space,
    bounding_box: areas::Box<U::Space>
}

impl<const D: usize, U: Universe<Space = space::Meters<D>, Mass = mass::Kilograms>> Body<U> for BasicEnemy<U> {
    type Boundary = areas::Box<U::Space>;

    fn boundary(&self) -> Self::Boundary {
        self.bounding_box
    }

    fn position(&self) -> U::Space {
        self.position
    }

    fn mass(&self) -> U::Mass {
        mass::Kilograms::new(5)
    }
}