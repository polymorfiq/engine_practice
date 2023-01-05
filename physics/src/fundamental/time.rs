use super::{Mobile, Divisible};

/// `Time` represents potential for change within the system.
/// 
/// The more `Mass` moves through `Space` within a shorter period of `Time`, the more "energy" it takes.
pub trait Time: Mobile + Divisible {}