use std::fmt;

use anyhow::anyhow;

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl From<Direction> for char {
    fn from(direction: Direction) -> Self {
        match direction {
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(Up),
            'v' => Ok(Down),
            '<' => Ok(Left),
            '>' => Ok(Right),
            _ => Err(anyhow!("invalid direction: {:?}", c)),
        }
    }
}
