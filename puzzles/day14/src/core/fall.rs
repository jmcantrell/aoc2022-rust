use std::ops::{Add, AddAssign};

use super::Location;

use Fall::*;
pub const FALLS: [Fall; 3] = [South, SouthWest, SouthEast];

#[derive(Debug, Clone, Copy)]
pub enum Fall {
    South,
    SouthWest,
    SouthEast,
}

impl Add<Fall> for Location {
    type Output = Self;

    fn add(self, fall: Fall) -> Self {
        self + match fall {
            South => Location::new(0, 1),
            SouthWest => Location::new(-1, 1),
            SouthEast => Location::new(1, 1),
        }
    }
}

impl AddAssign<Fall> for Location {
    fn add_assign(&mut self, fall: Fall) {
        *self = *self + fall;
    }
}
