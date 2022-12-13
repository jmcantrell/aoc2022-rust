use std::convert::TryFrom;
use std::fmt;

use anyhow::anyhow;

use super::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn offset(&self) -> Point {
        match self {
            Self::Up => Point { x: 0, y: 1 },
            Self::Down => Point { x: 0, y: -1 },
            Self::Left => Point { x: -1, y: 0 },
            Self::Right => Point { x: 1, y: 0 },
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Up => 'U',
                Self::Down => 'D',
                Self::Left => 'L',
                Self::Right => 'R',
            }
        )?;
        Ok(())
    }
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow!("invalid direction: {:?}", c)),
        }
    }
}
