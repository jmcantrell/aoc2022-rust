pub type Point = nalgebra::Point3<isize>;

pub mod cube;
pub use cube::*;

pub mod grid;
pub use grid::*;

pub mod direction;
pub use direction::*;
