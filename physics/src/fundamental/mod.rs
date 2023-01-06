mod space;
pub use space::{Space, Point, Positional, Quantifiable, Intersectable, Intersects};

mod time;
pub use time::Time;

mod mass;
pub use mass::Mass;

mod universe;
pub use universe::Universe;

mod properties;
pub use properties::{Comparable, Mobile};