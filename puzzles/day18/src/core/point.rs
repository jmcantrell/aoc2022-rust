use std::convert::TryFrom;
use std::ops::{Add, Sub};

use anyhow::Context;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Self> for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl TryFrom<&str> for Point {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        fn parse_int(s: &str) -> anyhow::Result<isize> {
            s.parse()
                .with_context(|| format!("invalid integer: {:?}", s))
        }

        let mut components = s
            .trim()
            .splitn(3, ',')
            .enumerate()
            .map(|(i, s)| parse_int(s).with_context(|| format!("component number {}", i + 1)));

        let x = components.next().context("missing x component")??;
        let y = components.next().context("missing y component")??;
        let z = components.next().context("missing z component")??;

        Ok(Self { x, y, z })
    }
}
