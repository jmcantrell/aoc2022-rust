pub type Coordinate = usize;
pub type Location = (Coordinate, Coordinate);
pub type Size = nalgebra::Vector2<Coordinate>;
pub type Grid<T> = nalgebra::DMatrix<T>;

pub mod direction;
pub use direction::*;

pub mod cube_net;
pub use cube_net::*;

pub mod board;
pub use board::*;

pub mod cube;
pub use cube::*;

pub mod map;
pub use map::*;

pub mod movement;
pub use movement::*;

pub mod tile;
pub use tile::*;

pub mod walk;
pub use walk::*;
