use std::fmt;

use anyhow::anyhow;

use super::CardinalDirection;

use CardinalDirection::*;
use Tile::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Floor,
    Wall,
    Trail(CardinalDirection),
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Floor),
            '#' => Ok(Wall),
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
                Floor => '.',
                Wall => '#',
                Trail(direction) => match direction {
                    North => '^',
                    South => 'v',
                    West => '<',
                    East => '>',
                },
            }
        )?;

        Ok(())
    }
}
