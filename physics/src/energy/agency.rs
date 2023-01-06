use crate::Universe;

pub trait Agency<U: Universe> {
    fn self_force(time: U::Time);
}