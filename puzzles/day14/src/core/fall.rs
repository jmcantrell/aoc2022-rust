use std::ops::{Add, AddAssign};

use nalgebra::Vector2;

use super::Location;

use Fall::*;
pub const FALLS: [Fall; 3] = [South, SouthWest, SouthEast];

#[derive(Debug, Clone, Copy)]
pub enum Fall {
    South,
    SouthWest,
    SouthEast,
}

impl Fall {
    pub fn vector(&self) -> Vector2<isize> {
        match self {
            South => Vector2::new(0, 1),
            SouthWest => Vector2::new(-1, 1),
            SouthEast => Vector2::new(1, 1),
        }
    }
}

impl Add<Fall> for Location {
    type Output = Self;

    fn add(self, fall: Fall) -> Self {
        self + fall.vector()
    }
}

impl AddAssign<Fall> for Location {
    fn add_assign(&mut self, fall: Fall) {
        *self = *self + fall;
    }
}
