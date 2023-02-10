use std::fmt;

use anyhow::anyhow;

use super::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Floor,
    Wall,
    Trail(Direction),
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Floor),
            '#' => Ok(Self::Wall),
            _ => Err(anyhow!("invalid tile: {:?}", c)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Floor => '.',
                Self::Wall => '#',
                Self::Trail(direction) => match direction {
                    Direction::North => '^',
                    Direction::South => 'v',
                    Direction::West => '<',
                    Direction::East => '>',
                },
            }
        )?;

        Ok(())
    }
}
