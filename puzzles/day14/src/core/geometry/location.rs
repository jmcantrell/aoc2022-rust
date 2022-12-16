use std::convert::TryFrom;
use std::ops::{Add, Sub};

use anyhow::Context;

use super::Direction;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location {
    pub row: isize,
    pub column: isize,
}

impl Location {
    pub fn unit(&self) -> Location {
        Self {
            row: self.row.signum(),
            column: self.column.signum(),
        }
    }
}

impl Add<Self> for Location {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}

impl Sub<Self> for Location {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            row: self.row - other.row,
            column: self.column - other.column,
        }
    }
}

impl Add<Direction> for Location {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        self + direction.unit()
    }
}

impl TryFrom<&str> for Location {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self, Self::Error> {
        fn parse_int(s: &str) -> anyhow::Result<isize> {
            s.parse()
                .with_context(|| format!("invalid integer: {:?}", s))
        }

        let mut words = s.trim().splitn(2, ",");

        let column = parse_int(words.next().context("missing column")?)?;
        let row = parse_int(words.next().context("missing row")?)?;

        Ok(Self { row, column })
    }
}
