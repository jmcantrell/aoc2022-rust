pub type Coordinate = usize;
pub type Location = (Coordinate, Coordinate);
pub type Grid<T> = nalgebra::DMatrix<T>;

pub mod direction;
pub use direction::*;

pub mod heightmap;
pub use heightmap::*;

pub mod breadcrumbs;
pub use breadcrumbs::*;
