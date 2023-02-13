pub type Grid<T> = nalgebra::DMatrix<T>;
pub type Location = (usize, usize);

pub mod direction;
pub use direction::*;

pub mod heightmap;
pub use heightmap::*;

pub mod breadcrumbs;
pub use breadcrumbs::*;
