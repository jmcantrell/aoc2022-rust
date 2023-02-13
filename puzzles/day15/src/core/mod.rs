pub type Point = nalgebra::Point2<isize>;

pub mod range;
pub use range::*;

pub mod sensor_grid;
pub use sensor_grid::*;

pub mod taxicab;
pub use taxicab::*;
