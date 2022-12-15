use std::ops::Add;

use super::Direction;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl Add<Direction> for Location {
    type Output = Option<Self>;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => {
                if self.row > 0 {
                    Some(Location {
                        row: self.row - 1,
                        column: self.column,
                    })
                } else {
                    None
                }
            }
            Direction::West => {
                if self.column > 0 {
                    Some(Location {
                        row: self.row,
                        column: self.column - 1,
                    })
                } else {
                    None
                }
            }
            Direction::East => Some(Location {
                row: self.row,
                column: self.column + 1,
            }),
            Direction::South => Some(Location {
                row: self.row + 1,
                column: self.column,
            }),
        }
    }
}
