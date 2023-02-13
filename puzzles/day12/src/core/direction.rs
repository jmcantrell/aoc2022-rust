use super::Location;

use Direction::*;
pub const DIRECTIONS: [Direction; 4] = [North, East, South, West];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn neighbor(&self, &(row, column): &Location) -> Option<Location> {
        match self {
            North => row.checked_sub(1).map(|row| (row, column)),
            South => row.checked_add(1).map(|row| (row, column)),
            West => column.checked_sub(1).map(|column| (row, column)),
            East => column.checked_add(1).map(|column| (row, column)),
        }
    }
}
