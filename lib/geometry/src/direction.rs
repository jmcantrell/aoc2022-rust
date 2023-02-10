use std::fmt;
use std::ops::{Add, AddAssign};

use super::Rotation;

use CardinalDirection::*;
use RelativeDirection::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    East,
    South,
    West,
    North,
}

impl CardinalDirection {
    pub fn inverse(&self) -> Self {
        match self {
            East => West,
            South => North,
            West => East,
            North => South,
        }
    }
}

impl Add<Rotation> for CardinalDirection {
    type Output = Self;

    fn add(self, rotation: Rotation) -> Self::Output {
        match rotation {
            Rotation::Left => match self {
                East => North,
                South => East,
                West => South,
                North => West,
            },
            Rotation::Right => match self {
                East => South,
                South => West,
                West => North,
                North => East,
            },
        }
    }
}

impl AddAssign<Rotation> for CardinalDirection {
    fn add_assign(&mut self, rotation: Rotation) {
        *self = *self + rotation;
    }
}

impl fmt::Display for CardinalDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                East => "east",
                South => "south",
                West => "west",
                North => "north",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl RelativeDirection {
    pub fn inverse(&self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl Add<RelativeDirection> for CardinalDirection {
    type Output = CardinalDirection;

    fn add(self, relative: RelativeDirection) -> Self {
        match relative {
            Up => self,
            Down => self.inverse(),
            Left => self + Rotation::Left,
            Right => self + Rotation::Right,
        }
    }
}

impl AddAssign<RelativeDirection> for CardinalDirection {
    fn add_assign(&mut self, relative: RelativeDirection) {
        *self = *self + relative;
    }
}
