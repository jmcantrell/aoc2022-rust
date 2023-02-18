pub type Location = (usize, usize);
pub type Grid<T> = nalgebra::DMatrix<T>;

pub mod direction;
pub use direction::*;

pub mod tree_patch;
pub use tree_patch::*;

pub mod visibility;
pub use visibility::*;

pub mod scenic_score;
pub use scenic_score::*;
