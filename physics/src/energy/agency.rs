use crate::Universe;

pub trait Agency<const D: usize, U: Universe<D>> {
    fn self_force(time: U::Time);
}