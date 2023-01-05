use core::marker::Copy;
use core::ops::{Add, Sub, Div, Mul};
use core::cmp::{Eq, Ord};

/// Divisibility is the minimum requirement that the fundamental dimensions have in common.
/// 
/// In order to refer to a dimension via "units", we must be able to compose a quantity of "units" to create bigger "units" (`Add`, `Sub`, `Div`, `Mul`)
/// 
/// In order to "divide" a dimension, the concepts of "bigger" and "smaller" quantities of that dimension need to exist (`Eq`, `Ord`)
/// 
/// `Sized` is here primarily because `Add`, `Sub` and whatnot require it
pub trait Divisible: Eq + Ord + Add + Sub + Div + Mul + Sized {}

// This is just to save us some sanity. "Mobility" of units allows us to pass them around, assume they're safe to copy and otherwise take them as parameters
/// 
// Has less to do with a philisophical view of Physics and more to do with practicality and safety
pub trait Mobile: Copy + Sized {}