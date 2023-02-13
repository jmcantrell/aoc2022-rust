use std::fmt;

use anyhow::Context;

use super::Direction;

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movement {
    pub direction: Direction,
    pub magnitude: isize,
}

impl Movement {
    pub fn new(direction: Direction, magnitude: isize) -> Self {
        Self {
            direction,
            magnitude,
        }
    }
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}",
            match self.direction {
                Up => 'U',
                Down => 'D',
                Left => 'L',
                Right => 'R',
            },
            self.magnitude
        )?;

        Ok(())
    }
}

impl TryFrom<&str> for Movement {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_int(s: &str) -> anyhow::Result<isize> {
            s.parse().with_context(|| format!("invalid integer: {s:?}"))
        }

        let mut words = s.split_whitespace();

        let direction = words.next().context("missing direction")?.try_into()?;
        let magnitude = parse_int(words.next().context("missing magnitude")?)?;

        Ok(Self {
            direction,
            magnitude,
        })
    }
}
