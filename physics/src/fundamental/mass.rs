use super::{Comparable, Mobile};

/// `Mass` is the fundamental dimension of Matter.
/// 
/// The quantity of Matter that is concentrated in a given `Space` is measured by units of `Mass`.
pub trait Mass: Mobile + Comparable { }