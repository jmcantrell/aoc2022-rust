use std::fmt;

use anyhow::anyhow;

use Tile::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Wall,
    Floor,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Wall => "wall",
                Floor => "floor",
            }
        )
    }
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Wall => '#',
            Floor => '.',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Wall),
            '.' => Ok(Floor),
            _ => Err(anyhow!("invalid tile: {:?}", c)),
        }
    }
}
