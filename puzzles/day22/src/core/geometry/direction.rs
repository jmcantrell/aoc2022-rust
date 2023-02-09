use std::fmt;
use std::ops::{Add, AddAssign};

use super::Rotation;

pub const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match self {
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::North => Self::South,
        }
    }
}

impl Add<Rotation> for Direction {
    type Output = Self;

    fn add(self, rotation: Rotation) -> Self::Output {
        match rotation {
            Rotation::Left => match self {
                Self::East => Self::North,
                Self::South => Self::East,
                Self::West => Self::South,
                Self::North => Self::West,
            },
            Rotation::Right => match self {
                Self::East => Self::South,
                Self::South => Self::West,
                Self::West => Self::North,
                Self::North => Self::East,
            },
        }
    }
}

impl AddAssign<Rotation> for Direction {
    fn add_assign(&mut self, rotation: Rotation) {
        *self = *self + rotation;
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::East => "east",
                Self::South => "south",
                Self::West => "west",
                Self::North => "north",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelativeDirection {
    Left,
    Right,
    Forward,
    Backward,
}

impl RelativeDirection {
    pub fn inverse(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
        }
    }
}

impl Add<RelativeDirection> for Direction {
    type Output = Direction;

    fn add(self, relative: RelativeDirection) -> Self {
        match relative {
            RelativeDirection::Forward => self,
            RelativeDirection::Backward => self.inverse(),
            RelativeDirection::Left => self + Rotation::Left,
            RelativeDirection::Right => self + Rotation::Right,
        }
    }
}

impl AddAssign<RelativeDirection> for Direction {
    fn add_assign(&mut self, relative: RelativeDirection) {
        *self = *self + relative;
    }
}
