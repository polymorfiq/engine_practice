use physics::Area;
use physics::units::space;

pub struct Box2 {
    top_left: space::Meters2,
    bottom_right: space::Meters2
}

impl Area<space::Meters2> for Box2 {
    fn intersects(&self, space: &space::Meters2) -> bool {
        false
    }
}