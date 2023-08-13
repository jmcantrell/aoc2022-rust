use std::fmt;
use std::ops::{Add, AddAssign};

use super::Location;

use CardinalDirection::*;
use RelativeDirection::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub fn value(&self) -> usize {
        match self {
            East => 0,
            South => 1,
            West => 2,
            North => 3,
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub fn neighbor(&self, (row, column): Location) -> Option<Location> {
        match self {
            North => row.checked_sub(1).map(|row| (row, column)),
            South => row.checked_add(1).map(|row| (row, column)),
            West => column.checked_sub(1).map(|column| (row, column)),
            East => column.checked_add(1).map(|column| (row, column)),
        }
    }
}

impl fmt::Display for CardinalDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                North => "north",
                East => "east",
                South => "south",
                West => "west",
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

impl Add<RelativeDirection> for CardinalDirection {
    type Output = Self;

    fn add(self, dir: RelativeDirection) -> Self {
        match dir {
            Forward => self,
            Backward => self.inverse(),
            Left => match self {
                North => West,
                West => South,
                South => East,
                East => North,
            },
            Right => match self {
                North => East,
                East => South,
                South => West,
                West => North,
            },
        }
    }
}

impl AddAssign<RelativeDirection> for CardinalDirection {
    fn add_assign(&mut self, dir: RelativeDirection) {
        *self = *self + dir;
    }
}
