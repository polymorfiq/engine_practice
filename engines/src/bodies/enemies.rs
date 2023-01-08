use physics::{Body, Universe};
use physics::space::BaseUnit;
use physics::units::{space, mass};
use crate::areas;

pub struct BasicEnemy<const D: usize, U: Universe<D>> {
    position: U::Space,
    bounding_box: areas::NonRotatingBox<D, U::Space>
}

impl<const D: usize, B: BaseUnit, U: Universe<D, Space = space::Meters<D, B>, Mass = mass::Kilograms>> Body<D, U> for BasicEnemy<D, U> {
    type Boundary = areas::NonRotatingBox<D, U::Space>;

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