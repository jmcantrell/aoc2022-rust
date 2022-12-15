use std::ops::Add;

use super::Location;

pub const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Add<Location> for Direction {
    type Output = Option<Location>;

    fn add(self, location: Location) -> Self::Output {
        location + self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_north() {
        let column = 5;
        let direction = Direction::North;
        assert_eq!(Location { row: 0, column } + direction, None);
        assert_eq!(
            Location { row: 1, column } + direction,
            Some(Location { row: 0, column })
        );
    }

    #[test]
    fn direction_east() {
        let row = 5;
        let direction = Direction::East;
        assert_eq!(
            Location { row, column: 0 } + direction,
            Some(Location { row, column: 1 })
        );
    }

    #[test]
    fn direction_south() {
        let column = 5;
        let direction = Direction::South;
        assert_eq!(
            Location { row: 0, column } + direction,
            Some(Location { row: 1, column })
        );
    }

    #[test]
    fn direction_west() {
        let row = 5;
        let direction = Direction::West;
        assert_eq!(Location { row, column: 0 } + direction, None);
        assert_eq!(
            Location { row, column: 1 } + direction,
            Some(Location { row, column: 0 })
        );
    }
}
