use super::Location;

pub const DIRECTIONS: [Direction; 3] =
    [Direction::South, Direction::SouthWest, Direction::SouthEast];

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    South,
    SouthWest,
    SouthEast,
}

impl Direction {
    pub fn unit(&self) -> Location {
        match self {
            Self::South => Location { row: 1, column: 0 },
            Self::SouthWest => Location { row: 1, column: -1 },
            Self::SouthEast => Location { row: 1, column: 1 },
        }
    }
}
