use super::Location;

use Direction::*;
pub const DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn neighbor(&self, (row, col): Location) -> Option<Location> {
        match self {
            North => row.checked_sub(1).map(|row| (row, col)),
            South => row.checked_add(1).map(|row| (row, col)),
            West => col.checked_sub(1).map(|col| (row, col)),
            East => col.checked_add(1).map(|col| (row, col))
        }
    }
}
