use physics::{Body, Universe};
use physics::space::BaseUnit;
use physics::units::{space, mass};
use crate::areas;

pub struct BasicEnemy<U: Universe> {
    position: U::Space,
    bounding_box: areas::NonRotatingBox<U::Space>
}

impl<B: BaseUnit, U: Universe<Space = space::Meters<2, B>, Mass = mass::Kilograms>> Body<U> for BasicEnemy<U> {
    type Boundary = areas::NonRotatingBox<U::Space>;

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